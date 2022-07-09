//! Module contains all the code for the new generation media scanner.

mod mediafile;
mod movie;
#[cfg(test)]
mod tests;
mod tv_show;

use self::mediafile::Error as CreatorError;
use self::mediafile::MediafileCreator;
use super::external::filename::FilenameMetadata;
use super::external::filename::Metadata;
use super::external::ExternalQuery;
use crate::core::EventTx;

use anitomy::Anitomy;
use async_trait::async_trait;

use database::library::Library;
use database::library::MediaType;
use database::mediafile::InsertableMediaFile;
use database::mediafile::MediaFile;

use futures::FutureExt;
use itertools::Itertools;

use std::ffi::OsStr;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use torrent_name_parser::Metadata as TorrentMetadata;
use tracing::info;
use tracing::instrument;
use tracing::warn;
use walkdir::WalkDir;

pub(super) static SUPPORTED_EXTS: &[&str] = &["mp4", "mkv", "avi", "webm"];

/// Function recursively walks the paths passed and returns all files in those directories.
/// FIXME: THIS IS NOT ASYNC-SAFE!!!
pub fn get_subfiles(paths: impl Iterator<Item = impl AsRef<Path>>) -> Vec<PathBuf> {
    let mut files = Vec::with_capacity(2048);
    for path in paths {
        let mut subfiles: Vec<PathBuf> = WalkDir::new(path)
            // we want to follow all symlinks in case of complex dir structures
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
            // ignore all hidden files.
            .filter(|f| {
                !f.path()
                    .iter()
                    .any(|s| s.to_str().map(|x| x.starts_with('.')).unwrap_or(false))
            })
            // check whether `f` has a supported extension
            .filter(|f| {
                f.path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .map_or(false, |e| SUPPORTED_EXTS.contains(&e))
            })
            .map(|f| f.into_path())
            .collect();

        files.append(&mut subfiles);
    }

    files
}

pub fn parse_filenames(
    files: impl Iterator<Item = impl AsRef<Path>>,
) -> Vec<(PathBuf, Vec<Metadata>)> {
    let mut metadata = Vec::new();

    for file in files {
        let filename = match file.as_ref().file_stem().and_then(OsStr::to_str) {
            Some(x) => x,
            None => {
                warn!(file = ?file.as_ref(), "Received a filename that is not unicode");
                continue;
            }
        };

        let metas = IntoIterator::into_iter([
            TorrentMetadata::from_str(&filename),
            Anitomy::from_str(&filename),
        ])
        .filter_map(|x| x)
        .collect::<Vec<_>>();

        if metas.is_empty() {
            warn!(file = ?file.as_ref(), "Failed to parse the filename and extract metadata.");
            continue;
        }

        metadata.push((file.as_ref().into(), metas));
    }

    metadata
}

pub struct WorkUnit(pub MediaFile, pub Vec<Metadata>);

/// Trait that must be implemented by a media matcher. Matchers are responsible for fetching their
/// own external metadata but it is provided a metadata provider at initialization time.
#[async_trait]
pub trait MediaMatcher {
    async fn batch_match(
        &self,
        tx: &mut database::Transaction<'_>,
        provider: Arc<dyn ExternalQuery>,
        work: Vec<WorkUnit>,
    );
}

pub async fn insert_mediafiles(
    conn: &mut database::DbConnection,
    library_id: i64,
    dirs: Vec<impl AsRef<Path>>,
) -> Result<Vec<WorkUnit>, Box<dyn std::error::Error>> {
    let subfiles = get_subfiles(dirs.into_iter());
    let parsed = parse_filenames(subfiles.iter());

    let mut instance = MediafileCreator::new(conn.clone(), library_id).await;

    let insertable_futures =
        parsed
            .clone()
            .into_iter()
            .map(|(path, meta)| instance.construct_mediafile(path, meta[0].clone()).boxed())
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect())
            .collect::<Vec<
                Vec<
                    Pin<Box<dyn Future<Output = Result<InsertableMediaFile, CreatorError>> + Send>>,
                >,
            >>();

    let mut insertables = vec![];

    for chunk in insertable_futures.into_iter() {
        let results: Vec<Result<InsertableMediaFile, CreatorError>> =
            futures::future::join_all(chunk).await;

        for result in results {
            insertables.push(result?);
        }
    }

    let mut mediafiles = vec![];

    for chunk in insertables.chunks(128) {
        mediafiles.append(&mut instance.insert_batch(chunk.iter()).await?);
    }

    Ok(mediafiles
        .into_iter()
        .zip(parsed.into_iter())
        .map(|(mfile, (_, metadata))| WorkUnit(mfile, metadata))
        .collect())
}

#[instrument(skip(conn, dirs, tx))]
pub async fn start_custom(
    conn: &mut database::DbConnection,
    library_id: i64,
    dirs: Vec<impl AsRef<Path>>,
    tx: EventTx,
    media_type: MediaType,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(library_id, "Scanning library");

    tx.send(
        events::Message {
            id: library_id,
            event_type: events::PushEventType::EventStartedScanning,
        }
        .to_string(),
    )?;

    let matcher = match media_type {
        MediaType::Movie => Arc::new(movie::MovieMatcher) as Arc<dyn MediaMatcher>,
        _ => unimplemented!(),
    };

    let provider = Arc::new(crate::external::mock::MockProvider);

    let now = Instant::now();
    let workunits = insert_mediafiles(conn, library_id, dirs).await?;

    info!(
        library_id,
        units = workunits.len(),
        elapsed_ms = now.elapsed().as_millis(),
        "Walked and inserted mediafiles."
    );

    let now = Instant::now();

    // TODO: We can receive work over a channel so that we can in parallel create new mediafiles
    // and match objects.
    for unit in workunits.into_iter().chunks(128).into_iter() {
        let mut lock = conn.writer().lock_owned().await;
        let mut tx = database::write_tx(&mut lock).await?;

        matcher
            .batch_match(&mut tx, provider.clone(), unit.collect())
            .await;

        tx.commit().await?;
    }

    info!(
        library_id,
        elapsed_ms = now.elapsed().as_millis(),
        "Finished scanning library."
    );

    tx.send(
        events::Message {
            id: library_id,
            event_type: events::PushEventType::EventStoppedScanning,
        }
        .to_string(),
    )?;

    Ok(())
}

pub async fn start(
    conn: &mut database::DbConnection,
    library_id: i64,
    tx: EventTx,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx_ = conn.read().begin().await?;

    let lib = Library::get_one(&mut tx_, library_id).await?;

    start_custom(conn, library_id, lib.locations, tx, lib.media_type).await
}

/// Function formats the path where assets are stored.
pub fn format_path(x: Option<String>) -> String {
    x.map(|x| format!("images/{}", x.trim_start_matches('/')))
        .unwrap_or_default()
}
