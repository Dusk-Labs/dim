use crate::core::EventTx;

use std::array::IntoIter;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use database::get_conn;
use database::library::Library;
use database::library::MediaType;
use database::media::Media;
use database::mediafile::MediaFile;
use database::mediafile::UpdateMediaFile;
use database::DbConnection;

use notify::DebouncedEvent;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;

use err_derive::Error;
use tokio::task::spawn_blocking;
use tracing::debug;
use tracing::error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum FsWatcherError {
    #[error(display = "A database error has occured")]
    DatabaseError(#[source] database::DatabaseError),
    #[error(display = "A error with notify has occured")]
    NotifyError(#[source] notify::Error),
}

pub struct FsWatcher {
    media_type: MediaType,
    library_id: i64,
    tx: EventTx,
    conn: DbConnection,
}

impl FsWatcher {
    pub async fn new(library_id: i64, media_type: MediaType, tx: EventTx) -> Self {
        Self {
            library_id,
            media_type,
            tx,
            conn: get_conn()
                .await
                .expect("Failed to grab the connection pool."),
        }
    }

    pub async fn start_daemon(&self) -> Result<(), FsWatcherError> {
        let library = Library::get_one(&self.conn, self.library_id).await?;

        let (tx, mut rx) = mpsc::channel();
        let mut watcher = <RecommendedWatcher as Watcher>::new(tx, Duration::from_secs(1))?;

        for location in &library.locations {
            watcher.watch(location.as_str(), RecursiveMode::Recursive)?;
        }

        loop {
            // NOTE: God forgive me
            let (_rx, result) = spawn_blocking(move || {
                let rx = rx;
                let res = rx.recv();
                (rx, res)
            })
            .await
            .unwrap();

            rx = _rx;

            match result {
                Ok(DebouncedEvent::Create(path)) => self.handle_create(path).await,
                Ok(DebouncedEvent::Rename(from, to)) => self.handle_rename(from, to).await,
                Ok(DebouncedEvent::Remove(path)) => self.handle_remove(path).await,
                Ok(event) => debug!("Tried to handle unmatched event {:?}", event),
                Err(e) => error!("Received error: {:?}", e),
            }
        }
    }

    async fn handle_create(&self, path: PathBuf) {
        debug!("Received handle_create event type: {:?}", path);

        if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| super::SUPPORTED_EXTS.contains(&e))
        {
            let extractor = super::get_extractor(&&self.tx);
            let matcher = super::get_matcher(&&self.tx);

            if let Ok(mfile) = extractor
                .mount_file(path.clone(), self.library_id, self.media_type)
                .await
            {
                match self.media_type {
                    MediaType::Movie => {
                        let _ = matcher.match_movie(mfile).await;
                    }
                    MediaType::Tv => {
                        let _ = matcher.match_tv(mfile).await;
                    }
                    _ => unreachable!(),
                }
            }
        } else if path.is_dir() {
            if let Some(x) = path.to_str() {
                let _ = super::start_custom(
                    self.library_id,
                    self.tx.clone(),
                    IntoIter::new([x]),
                    self.media_type,
                )
                .await;
            }
        }
    }

    async fn handle_remove(&self, path: PathBuf) {
        debug!("Received handle remove {:?}", path);

        let path = match path.to_str() {
            Some(x) => x,
            None => {
                warn!(
                    "Received path thats not unicode {}",
                    path = format!("{:?}", path)
                );
                return;
            }
        };

        if let Some(media_file) = MediaFile::get_by_file(&self.conn, path).await.ok() {
            let media = Media::get_of_mediafile(&self.conn, media_file.id).await;

            if let Err(e) = MediaFile::delete(&self.conn, media_file.id).await {
                error!("Failed to remove mediafile {}", reason = format!("{:?}", e));
                return;
            }

            // if we have a media with no mediafiles we want to purge it as it is a ghost media
            // entry.
            if let Ok(media) = media {
                if let Ok(media_files) = MediaFile::get_of_media(&self.conn, media.id).await {
                    if media_files.is_empty() {
                        if let Err(e) = Media::delete(&self.conn, media.id).await {
                            error!("Failed to delete ghost media {:?}", e);
                            return;
                        }
                    }
                }
            }
        }
    }

    async fn handle_rename(&self, from: PathBuf, to: PathBuf) {
        debug!(
            "Received handle rename {}/{}",
            from = format!("{:?}", from),
            to = format!("{:?}", to),
        );

        let from = match from.to_str() {
            Some(x) => x,
            None => {
                warn!(
                    "Received path thats not unicode {}",
                    path = format!("{:?}", from)
                );
                return;
            }
        };

        let to = match to.to_str() {
            Some(x) => x,
            None => {
                warn!(
                    "Received path thats not unicode {}",
                    path = format!("{:?}", to)
                );
                return;
            }
        };

        if let Some(media_file) = MediaFile::get_by_file(&self.conn, from).await.ok() {
            let update_query = UpdateMediaFile {
                target_file: Some(to.into()),
                ..Default::default()
            };

            if let Err(_e) = update_query.update(&self.conn, media_file.id).await {
                error!(
                    "Failed to update target file {}/{}/{}",
                    from = format!("{:?}", from),
                    to = format!("{:?}", to),
                    mediafile_id = media_file.id
                );
            }
        }
    }
}
