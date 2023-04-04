use crate::core::EventTx;
use crate::external::ExternalQueryIntoShow;

use super::movie;
use super::tv_show;
use super::MediaMatcher;

use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;

use dim_database::library::Library;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::mediafile::UpdateMediaFile;
use dim_database::DbConnection;

use notify::Config;
use notify::EventKind;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;

use notify::event::ModifyKind;
use notify::event::RenameMode;
use tokio::sync::mpsc::UnboundedReceiver;

use displaydoc::Display;
use thiserror::Error;
use tracing::{debug, error, warn};

#[derive(Display, Debug, Error)]
pub enum FsWatcherError {
    /// A database error has occured: {0:?}
    DatabaseError(#[from] dim_database::DatabaseError),
    /// A error with notify has occured": {0:?}
    NotifyError(#[from] notify::Error),
}

pub struct FsWatcher {
    media_type: MediaType,
    library_id: i64,
    tx: EventTx,
    conn: DbConnection,
    matcher: Arc<dyn MediaMatcher>,
    provider: Arc<dyn ExternalQueryIntoShow>,
}

impl FsWatcher {
    pub fn new(
        conn: DbConnection,
        library_id: i64,
        media_type: MediaType,
        tx: EventTx,
        provider: Arc<dyn ExternalQueryIntoShow>,
    ) -> Self {
        let matcher = match media_type {
            MediaType::Movie => Arc::new(movie::MovieMatcher) as Arc<dyn MediaMatcher>,
            MediaType::Tv => Arc::new(tv_show::TvMatcher) as Arc<dyn MediaMatcher>,
            _ => unimplemented!(),
        };

        Self {
            library_id,
            media_type,
            tx,
            conn,
            matcher,
            provider,
        }
    }

    pub async fn start_daemon(&mut self) -> Result<(), FsWatcherError> {
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

        let (mut rx, _watcher) = spawn_file_watcher(library.locations.as_slice())?;

        while let Some(ev) = rx.recv().await {
            let mut ev = match ev {
                Ok(ev) => ev,
                Err(err) => {
                    error!(?err, "notify event error");
                    continue;
                }
            };

            match ev.kind {
                EventKind::Create(_) => {
                    for path in ev.paths {
                        self.handle_create(path).await
                    }
                }
                EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                    if ev.paths.len() != 2 {
                        debug!(paths = ?ev.paths, "rename event with both names does not contain exactly two paths");
                        continue;
                    } else {
                        assert!(ev.paths.len() == 2);
                    }
                    let [from, to] = [(); 2].map(|()| ev.paths.remove(0));
                    self.handle_rename(from, to).await
                }
                EventKind::Remove(_) => {
                    for path in ev.paths {
                        self.handle_remove(path).await
                    }
                }
                event => debug!("Tried to handle unmatched event {:?}", event),
            }
        }

        warn!(library_id = self.library_id, "Scanning daemon finished.");

        Ok(())
    }

    async fn handle_create(&mut self, path: PathBuf) {
        debug!("Received handle_create event type: {:?}", path);

        if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| super::SUPPORTED_EXTS.contains(&e))
        {
            if let Ok(mfile) =
                super::insert_mediafiles(&mut self.conn, self.library_id, vec![path.clone()]).await
            {
                let mut lock = self.conn.writer().lock_owned().await;
                let mut tx = dim_database::write_tx(&mut lock).await.unwrap();

                if let Err(e) = self
                    .matcher
                    .batch_match(&mut tx, self.provider.clone(), mfile)
                    .await
                {
                    error!(error=?e, "Failed to match new file");
                    return;
                }

                tx.commit().await.unwrap();
            }
        } else if path.is_dir() {
            if let Some(x) = path.to_str() {
                let _ = super::start_custom(
                    &mut self.conn,
                    self.library_id,
                    vec![x.to_string()],
                    self.tx.clone(),
                    self.media_type,
                    self.provider.clone(),
                )
                .await;
            }
        }
    }

    async fn handle_remove(&mut self, path: PathBuf) {
        debug!("Received handle remove {:?}", path);

        let path = match path.to_str() {
            Some(x) => x,
            None => {
                warn!(?path, "Received path thats not unicode",);
                return;
            }
        };

        let mut lock = self.conn.writer().lock_owned().await;
        let mut tx = match dim_database::write_tx(&mut lock).await {
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

    async fn handle_rename(&mut self, from: PathBuf, to: PathBuf) {
        debug!(
            from = ?from,
            to = ?to,
            "Received handle rename",
        );

        let from = match from.to_str() {
            Some(x) => x,
            None => {
                warn!(
                    path=?from,
                    "Received path thats not unicode",
                );
                return;
            }
        };

        let to = match to.to_str() {
            Some(x) => x,
            None => {
                warn!(
                    path=?to,
                    "Received path thats not unicode",
                );
                return;
            }
        };

        let mut lock = self.conn.writer().lock_owned().await;
        let mut tx = match dim_database::write_tx(&mut lock).await {
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

pub fn spawn_file_watcher<S>(
    paths: &[S],
) -> Result<
    (
        UnboundedReceiver<notify::Result<notify::Event>>,
        RecommendedWatcher,
    ),
    FsWatcherError,
>
where
    S: AsRef<str>,
{
    let (tx, rx) = mpsc::channel();
    let mut watcher = <RecommendedWatcher as Watcher>::new(tx, Config::default())?;

    for path in paths {
        watcher.watch(
            std::path::Path::new(path.as_ref()),
            RecursiveMode::Recursive,
        )?;
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
