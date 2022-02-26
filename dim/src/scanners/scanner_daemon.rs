use crate::core::EventTx;

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

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

use tokio::sync::mpsc::UnboundedReceiver;

use thiserror::Error;
use displaydoc::Display;
use tracing::debug;
use tracing::error;
use tracing::warn;

#[derive(Display, Debug, Error)]
pub enum FsWatcherError {
    /// A database error has occured: {0:?}
    DatabaseError(#[from] database::DatabaseError),
    /// A error with notify has occured": {0:?}
    NotifyError(#[from] notify::Error),
}

pub struct FsWatcher {
    media_type: MediaType,
    library_id: i64,
    tx: EventTx,
    conn: DbConnection,
}

impl FsWatcher {
    pub fn new(conn: DbConnection, library_id: i64, media_type: MediaType, tx: EventTx) -> Self {
        Self {
            library_id,
            media_type,
            tx,
            conn,
        }
    }

    pub async fn start_daemon(&self) -> Result<(), FsWatcherError> {
        let library = {
            let mut tx = match self.conn.read().begin().await {
                Ok(x) => x,
                Err(e) => {
                    error!(reason = ?e, "Failed to open a transaction.");
                    return Ok(());
                }
            };

            Library::get_one(&mut tx, self.library_id).await?
        };

        let (mut rx, _watcher) = async_watch(library.locations.iter())?;

        while let Some(e) = rx.recv().await {
            match e {
                DebouncedEvent::Create(path) => self.handle_create(path).await,
                DebouncedEvent::Rename(from, to) => self.handle_rename(from, to).await,
                DebouncedEvent::Remove(path) => self.handle_remove(path).await,
                event => debug!("Tried to handle unmatched event {:?}", event),
            }
        }

        warn!(library_id = self.library_id, "Scanning daemon finished.");

        Ok(())
    }

    async fn handle_create(&self, path: PathBuf) {
        debug!("Received handle_create event type: {:?}", path);

        if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| super::SUPPORTED_EXTS.contains(&e))
        {
            let extractor = super::get_extractor(&self.tx);
            let matcher = super::get_matcher(&self.tx);

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
                    IntoIterator::into_iter([x]),
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

        let mut lock = self.conn.writer().lock_owned().await;
        let mut tx = match database::write_tx(&mut lock).await {
            Ok(x) => x,
            Err(e) => {
                error!(reason = ?e, "Failed to create transaction.");
                return;
            }
        };

        if let Ok(media_file) = MediaFile::get_by_file(&mut tx, path).await {
            let media = Media::get_of_mediafile(&mut tx, media_file.id).await;

            if let Err(e) = MediaFile::delete(&mut tx, media_file.id).await {
                error!(reason = ?e, "Failed to remove mediafile");
                return;
            }

            // if we have a media with no mediafiles we want to purge it as it is a ghost media
            // entry.
            if let Ok(media) = media {
                if let Ok(media_files) = MediaFile::get_of_media(&mut tx, media.id).await {
                    if media_files.is_empty() {
                        if let Err(e) = Media::delete(&mut tx, media.id).await {
                            error!(reason = ?e, "Failed to delete ghost media");
                            return;
                        }
                    }
                }
            }

            if let Err(e) = tx.commit().await {
                error!(reason = ?e, "Failed to commit transaction.");
            }
        }
    }

    async fn handle_rename(&self, from: PathBuf, to: PathBuf) {
        debug!(
            from = ?from,
            to = ?to,
            "Received handle rename",
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

        let mut lock = self.conn.writer().lock_owned().await;
        let mut tx = match database::write_tx(&mut lock).await {
            Ok(x) => x,
            Err(e) => {
                error!(reason = ?e, "Failed to create transaction.");
                return;
            }
        };

        if let Ok(media_file) = MediaFile::get_by_file(&mut tx, from).await {
            let update_query = UpdateMediaFile {
                target_file: Some(to.into()),
                ..Default::default()
            };

            if let Err(_e) = update_query.update(&mut tx, media_file.id).await {
                error!(
                    from = ?from,
                    to = ?to,
                    mediafile_id = media_file.id,
                    "Failed to update target file",
                );
            }

            if let Err(e) = tx.commit().await {
                error!(reason = ?e, "Failed to commit transaction.");
            }
        }
    }
}

// FIXME(val): This code is pretty cursed. We should replace this with native async when notify==5.0.0
// comes out.
pub fn async_watch(
    paths: impl Iterator<Item = impl AsRef<str>>,
) -> Result<(UnboundedReceiver<DebouncedEvent>, RecommendedWatcher), FsWatcherError> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = <RecommendedWatcher as Watcher>::new(tx, Duration::from_secs(5))?;

    for path in paths {
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    }

    let (async_tx, async_rx) = tokio::sync::mpsc::unbounded_channel();

    std::thread::spawn(move || {
        while let Ok(x) = rx.recv() {
            if async_tx.send(x).is_err() {
                break;
            }
        }
    });

    Ok((async_rx, watcher))
}
