use err_derive::Error;
use std::path::Path;
use std::path::PathBuf;

use database::library::MediaType;
use database::mediafile::InsertableMediaFile;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;
use database::DbConnection;

use crate::core::EventTx;
use crate::scanners::movie::MovieMatcher;
use crate::scanners::tmdb::Tmdb;
use crate::scanners::tv_show::TvShowMatcher;
use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::FFPROBE_BIN;

use super::ApiMedia;

use torrent_name_parser::Metadata;

use slog::debug;
use slog::error;
use slog::info;
use slog::o;
use slog::warn;

use serde::Serialize;

use tokio::task::spawn_blocking;

use async_trait::async_trait;
use xtra_proc::actor;
use xtra_proc::handler;

use anitomy::Anitomy;
use anitomy::ElementCategory;
use anitomy::Elements;

#[derive(Debug, Error, Serialize, Clone)]
pub enum ScannerError {
    #[error(display = "Could not get a connection to the db")]
    DatabaseConnectionError,
    #[error(display = "The filename parser returned no useful results")]
    FilenameParserError,
    #[error(display = "Something happened to ffprobe")]
    FFProbeError,
    #[error(display = "An unknown error has occured")]
    UnknownError,
    #[error(display = "Database error why={}", _0)]
    DatabaseError(String),
}

impl From<database::DatabaseError> for ScannerError {
    fn from(e: database::DatabaseError) -> Self {
        match e {
            database::DatabaseError::DatabaseError(e) => Self::DatabaseError(e.to_string()),
        }
    }
}

/// `MetadataExtractor` is an actor that processes files on the local filesystem. It parses the
/// filename to extract basic information such as title, year, episode/season. This actor will also
/// run ffprobe on the files to extract other metadata like format and codec.
///
/// Once a file is parsed and inserted into the database, it is sent to a `MetadataMatcher` actor.
/// Which will query extra external metadata from various APIs.
#[actor]
pub struct MetadataExtractor {
    pub conn: DbConnection,
    pub logger: slog::Logger,
}

#[actor]
impl MetadataExtractor {
    pub fn new(logger: slog::Logger) -> Self {
        Self {
            conn: database::try_get_conn().unwrap().clone(),
            logger: logger.new(o!("actor" => "MetadataExtractor")),
        }
    }

    #[handler]
    pub async fn mount_file(
        &mut self,
        file: PathBuf,
        library_id: i64,
        media_type: MediaType,
    ) -> Result<MediaFile, ScannerError> {
        let target_file = file.to_str().unwrap().to_owned();

        let file_name = if let Some(file_name) = file.file_name().and_then(|x| x.to_str()) {
            file_name
        } else {
            warn!(
                self.logger,
                "Received non-unicode filename";
                "file" => target_file,
            );
            return Err(ScannerError::UnknownError);
        };

        let target_file_clone = target_file.clone();
        let res = MediaFile::get_by_file(&self.conn, &target_file_clone).await;

        if let Ok(media_file) = res {
            debug!(
                self.logger,
                "File already exists in the db";
                "file" => file.to_string_lossy().to_string(),
                "library_id" => library_id,
            );
            return Err(ScannerError::UnknownError);
        }

        let ctx = FFProbeCtx::new(&FFPROBE_BIN);

        // we clone so that we can strip the extension.
        let mut file_name_clone = file.to_owned();
        file_name_clone.set_extension("");
        // unwrap will never panic because we validate the path earlier on.
        let file_name_clone = file_name_clone
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let clone = file_name_clone.clone();

        // closure needs to be bound because of a lifetime bug where the closure passed to
        // `spawn_blocking` lives more than the data moved into it thus we cant pass a reference to
        // `Metadata::from` directly.
        let meta_from_string =
            move || Metadata::from(&clone).map_err(|_| ScannerError::FilenameParserError);

        let metadata = match spawn_blocking(meta_from_string).await {
            Ok(x) => x?,
            Err(e) => {
                error!(self.logger, "Metadata::from possibly panic'd"; "e" => e.to_string());
                return Err(ScannerError::UnknownError);
            }
        };

        let ffprobe_data = if let Ok(data) = ctx.get_meta(&file) {
            data
        } else {
            error!(
                self.logger,
                "Couldnt extract media information with ffprobe";
                "file" => file.to_string_lossy().to_string(),
            );
            return Err(ScannerError::FFProbeError);
        };

        let media_file = InsertableMediaFile {
            library_id,
            media_id: None,
            target_file: target_file.to_string(),

            raw_name: metadata.title().to_owned(),
            raw_year: metadata.year().map(|x| x as i64),
            season: metadata.season().map(|x| x as i64),
            episode: metadata.episode().map(|x| x as i64),

            quality: ffprobe_data.get_quality(),
            codec: ffprobe_data.get_codec(),
            container: ffprobe_data.get_container(),
            audio: ffprobe_data.get_audio_type(),
            original_resolution: ffprobe_data.get_res(),
            duration: ffprobe_data.get_duration().map(|x| x as i64),
            corrupt: ffprobe_data.is_corrupt(),
        };

        let file_id = media_file.insert(&self.conn).await?;

        let id = MediaFile::get_one(&self.conn, file_id).await?;

        assert!(file_id == id.id);

        info!(
            self.logger,
            "Scanned file";
            "file" => &target_file,
            "library_id" => library_id,
            "id" => file_id,
            "2nd_pass_id" => id.id,
            "season" => metadata.season().unwrap_or(0),
            "episode" => metadata.episode().unwrap_or(0),
        );

        Ok(id)
    }
}

#[actor]
pub struct MetadataMatcher {
    pub movie_tmdb: Tmdb,
    pub tv_tmdb: Tmdb,
    pub log: slog::Logger,
    pub conn: DbConnection,
    pub event_tx: EventTx,
}

#[actor]
impl MetadataMatcher {
    pub fn new(log: slog::Logger, conn: DbConnection, event_tx: EventTx) -> Self {
        Self {
            conn,
            event_tx,
            movie_tmdb: Tmdb::new("38c372f5bc572c8aadde7a802638534e".into(), MediaType::Movie),
            tv_tmdb: Tmdb::new("38c372f5bc572c8aadde7a802638534e".into(), MediaType::Tv),
            log: log.new(o!("actor" => "MetadataMatcher")),
        }
    }

    #[handler]
    pub async fn match_movie(&mut self, media: MediaFile) -> Result<(), ScannerError> {
        let result = match self
            .movie_tmdb
            .search(media.raw_name.clone(), media.raw_year.map(|x| x as i32))
            .await
        {
            Ok(v) => v,
            Err(e) => {
                error!(
                    self.log,
                    "Could not match movie to tmdb";
                    "reason" => e.to_string(),
                );
                return Err(ScannerError::UnknownError);
            }
        };

        self.match_movie_to_result(media, result).await
    }

    #[handler]
    pub async fn match_movie_to_result(
        &mut self,
        media: MediaFile,
        result: ApiMedia,
    ) -> Result<(), ScannerError> {
        let matcher = MovieMatcher {
            conn: &self.conn,
            log: &self.log,
            event_tx: &self.event_tx,
        };

        matcher.match_to_result(result, &media).await;
        Ok(())
    }

    #[handler]
    pub async fn match_tv(&mut self, media: MediaFile) -> Result<(), ScannerError> {
        let mut media = media;

        let path = Path::new(&media.target_file);
        let filename = path
            .file_name()
            .and_then(|x| x.to_str())
            .map(ToString::to_string)
            .unwrap_or_default();

        let els: Elements = spawn_blocking(move || {
            let mut anitomy = Anitomy::new();
            anitomy.parse(filename.as_str())
        })
        .await
        .unwrap()
        .into_ok_or_err();

        let mut result = self
            .tv_tmdb
            .search(media.raw_name.clone(), media.raw_year.map(|x| x as i32))
            .await;

        if let Some(x) = els.get(ElementCategory::AnimeTitle) {
            if result.is_err() {
                // NOTE: If we got here then we assume that the file uses common anime release naming schemes.
                // Thus we prioritise metadata extracted by anitomy.
                result = self.tv_tmdb.search(x.to_string(), None).await;

                // NOTE: Some releases dont include season number, so we just assume its the first one.
                let anitomy_episode = els
                    .get(ElementCategory::EpisodeNumber)
                    .and_then(|x| x.parse::<i64>().ok())
                    .or(media.episode);

                let anitomy_season = els
                    .get(ElementCategory::AnimeSeason)
                    .and_then(|x| x.parse::<i64>().ok())
                    .or(Some(1));

                let update_mediafile = UpdateMediaFile {
                    episode: anitomy_episode.map(|x| x as i64),
                    season: anitomy_season.map(|x| x as i64),
                    raw_name: Some(x.to_string()),
                    ..Default::default()
                };

                let _ = update_mediafile.update(&self.conn, media.id).await;

                media.episode = anitomy_episode.map(|x| x as i64);
                media.season = anitomy_season.map(|x| x as i64);
            }
        }

        let result = match result {
            Ok(v) => v,
            Err(e) => {
                error!(
                    self.log,
                    "Could not match tv show to tmdb";
                    "reason" => e.to_string(),
                );
                return Err(ScannerError::UnknownError);
            }
        };

        self.match_tv_to_result(media, result).await
    }

    #[handler]
    pub async fn match_tv_to_result(
        &mut self,
        media: MediaFile,
        result: ApiMedia,
    ) -> Result<(), ScannerError> {
        // FIXME: Our handler macro cant handle `mut` keyword yet.
        let mut media = media;
        let mut result = result;

        let path = Path::new(&media.target_file);
        let filename = path
            .file_name()
            .and_then(|x| x.to_str())
            .map(ToString::to_string)
            .unwrap_or_default();

        let els: Elements = spawn_blocking(move || {
            let mut anitomy = Anitomy::new();
            anitomy.parse(filename.as_str())
        })
        .await
        .unwrap()
        .into_ok_or_err();

        if media.episode.is_none() {
            // NOTE: In some cases our base matcher extracts the correct title from the filename but incorrect episode and season numbers.
            let anitomy_episode = els
                .get(ElementCategory::EpisodeNumber)
                .and_then(|x| x.parse::<i64>().ok())
                .or(media.episode);

            let updated_mediafile = UpdateMediaFile {
                episode: anitomy_episode.map(|x| x as i64),
                ..Default::default()
            };

            let _ = updated_mediafile.update(&self.conn, media.id).await;
            media.episode = anitomy_episode.map(|x| x as i64);
        }

        if media.season.is_none() {
            // NOTE: Some releases dont include season number, so we just assume its the first one.
            let anitomy_season = els
                .get(ElementCategory::AnimeSeason)
                .and_then(|x| x.parse::<i32>().ok())
                .or(Some(1));

            let updated_mediafile = UpdateMediaFile {
                season: anitomy_season.map(|x| x as i64),
                ..Default::default()
            };

            let _ = updated_mediafile.update(&self.conn, media.id).await;
            media.season = anitomy_season.map(|x| x as i64);
        }

        let mut seasons: Vec<super::ApiSeason> = self
            .tv_tmdb
            .get_seasons_for(result.id)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        for season in seasons.iter_mut() {
            season.episodes = self
                .tv_tmdb
                .get_episodes_for(result.id, season.season_number)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect();
        }

        result.seasons = seasons;

        let matcher = TvShowMatcher {
            conn: &self.conn,
            log: &self.log,
            event_tx: &self.event_tx,
        };

        matcher.match_to_result(result, &media).await;
        Ok(())
    }

    #[handler]
    pub async fn match_anime(&mut self, media: MediaFile) -> Result<(), ScannerError> {
        Ok(())
    }
}
