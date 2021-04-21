use err_derive::Error;
use std::path::PathBuf;

use database::library::MediaType;
use database::mediafile::InsertableMediaFile;
use database::mediafile::MediaFile;
use database::DbConnection;

use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::FFPROBE_BIN;

use torrent_name_parser::Metadata;

use slog::debug;
use slog::error;
use slog::info;

use serde::Serialize;

use tokio::task::spawn_blocking;

use async_trait::async_trait;
use xtra_proc::actor;
use xtra_proc::handler;

#[derive(Debug, Error, Serialize)]
pub enum ScannerError {
    #[error(display = "Could not get a connection to the db")]
    DatabaseConnectionError,
    #[error(display = "The filename parser returned no useful results")]
    FilenameParserError,
    #[error(display = "Something happened to ffprobe")]
    FFProbeError,
    #[error(display = "An unknown error has occured")]
    UnknownError,
}

/// `MetadataExtractor` is an actor that processes files on the local filesystem. It parses the
/// filename to extract basic information such as title, year, episode/season. This actor will also
/// run ffprobe on the files to extract other metadata like format and codec.
///
/// Once a file is parsed and inserted into the database, it is sent to a `MetadataMatcher` actor.
/// Which will query extra external metadata from various APIs.
#[actor]
pub struct MetadataExtractor {
    pub conn: Option<DbConnection>,
    pub logger: slog::Logger,
}

#[actor]
impl MetadataExtractor {
    pub fn new(logger: slog::Logger, meta_matcher: MetadataMatcher) -> Self {
        Self {
            conn: Some(database::get_conn().unwrap()),
            logger,
        }
    }

    #[handler]
    pub async fn mount_file(
        &mut self,
        file: PathBuf,
        library_id: i32,
        media_type: MediaType,
    ) -> Result<MediaFile, ScannerError> {
        let target_file = file.to_str().unwrap().to_owned();
        // Assuming our hygiene is good and we always call `set_conn`, this should never panic.
        let conn = self.take_conn().unwrap();

        let file_name = if let Some(file_name) = file.file_name().and_then(|x| x.to_str()) {
            file_name
        } else {
            error!(
                self.logger,
                "Looks like file={:?} either has a non-unicode file_name, skipping.", file
            );
            self.set_conn(conn);
            return Err(ScannerError::UnknownError);
        };

        let target_file_clone = target_file.clone();
        let (res, conn) =
            spawn_blocking(move || (MediaFile::get_by_file(&conn, &target_file_clone), conn))
                .await
                .unwrap();
        self.set_conn(conn);

        if let Ok(media_file) = res {
            debug!(
                self.logger,
                "Tried to mount file that has already been mounted lib_id={} file_path={:?}",
                library_id,
                file
            );
            return Ok(media_file);
        }

        info!(
            self.logger,
            "Scanning file: {} for lib={}", &target_file, library_id
        );

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
        let metadata = spawn_blocking(meta_from_string).await.unwrap()?;

        let ffprobe_data = if let Ok(data) = ctx.get_meta(&file) {
            data
        } else {
            error!(self.logger, "Couldnt get data from ffprobe for file={:?}, this could be caused by ffprobe not existing", file);
            return Err(ScannerError::FFProbeError);
        };

        let media_file = InsertableMediaFile {
            library_id,
            media_id: None,
            target_file: target_file.to_string(),

            raw_name: metadata.title().to_owned(),
            raw_year: metadata.year(),
            season: metadata.season(),
            episode: metadata.episode(),

            quality: ffprobe_data.get_quality(),
            codec: ffprobe_data.get_codec(),
            container: ffprobe_data.get_container(),
            audio: ffprobe_data.get_audio_type(),
            original_resolution: ffprobe_data.get_res(),
            duration: ffprobe_data.get_duration(),
            corrupt: ffprobe_data.is_corrupt(),
        };

        let conn = self.take_conn().unwrap();
        let (file_id, conn) = spawn_blocking(move || (media_file.insert(&conn), conn))
            .await
            .unwrap();

        self.set_conn(conn);

        let file_id = file_id.map_err(|_| ScannerError::UnknownError)?;

        let conn = self.take_conn().unwrap();
        let (id, conn) = spawn_blocking(move || (MediaFile::get_one(&conn, file_id), conn))
            .await
            .unwrap();

        self.set_conn(conn);
        Ok(id.map_err(|_| ScannerError::UnknownError)?)
    }

    fn take_conn(&mut self) -> Option<DbConnection> {
        self.conn.take()
    }

    fn set_conn(&mut self, conn: DbConnection) {
        self.conn = Some(conn);
    }
}

#[actor]
pub struct MetadataMatcher;

#[actor]
impl MetadataMatcher {
    pub fn new() -> Self {
        Self
    }
}
