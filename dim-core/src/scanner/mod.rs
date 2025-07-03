//! Module contains all the code for the new generation media scanner.

pub mod daemon;
pub mod error;
mod mediafile;
pub mod movie;
#[cfg(test)]
mod tests;
pub mod tv_show;

use self::mediafile::Error as CreatorError;
use self::mediafile::MediafileCreator;
use crate::core::EventTx;

use async_trait::async_trait;

use dim_database::library::Library;
use dim_database::library::MediaType;
use dim_database::mediafile::InsertableMediaFile;
use dim_database::mediafile::MediaFile;

use dim_extern_api::filename::Anitomy;
use dim_extern_api::filename::CombinedExtractor;
use dim_extern_api::filename::FilenameMetadata;
use dim_extern_api::filename::Metadata;
use dim_extern_api::filename::TorrentMetadata;
use dim_extern_api::ExternalQueryIntoShow;

use futures::FutureExt;
use ignore::WalkBuilder;
use itertools::Itertools;

use std::ffi::OsStr;
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use tracing::error;
use tracing::info;
use tracing::instrument;
use tracing::warn;

pub use error::Error;

pub(super) static SUPPORTED_EXTS: &[&str] = &[
    "001",
    "3g2",
    "3gp",
    "amv",
    "asf",
    "asx",
    "avi",
    "bin",
    "bivx",
    "divx",
    "dv",
    "dvr-ms",
    "f4v",
    "fli",
    "flv",
    "ifo",
    "img",
    "iso",
    "m2t",
    "m2ts",
    "m2v",
    "m4v",
    "mkv",
    "mk3d",
    "mov",
    "mp4",
    "mpe",
    "mpeg",
    "mpg",
    "mts",
    "mxf",
    "nrg",
    "nsv",
    "nuv",
    "ogg",
    "ogm",
    "ogv",
    "pva",
    "qt",
    "rec",
    "rm",
    "rmvb",
    "strm",
    "svq3",
    "tp",
    "ts",
    "ty",
    "viv",
    "vob",
    "vp3",
    "webm",
    "wmv",
    "wtv",
    "xvid"
];

/// Function recursively walks the paths passed and returns all files in those directories.
/// FIXME: THIS IS NOT ASYNC-SAFE!!!
/// NOTE: I've noticed that walking a directory mounted over ssh is very slow, 80 files in like 300
/// seconds. Doubt theres a way to fix this but we could alliviate the UX-degradation by sending
/// the files over a channel instead of returning them at once.
pub fn get_subfiles(paths: impl Iterator<Item = impl AsRef<Path>>) -> Vec<PathBuf> {
    let mut files = Vec::with_capacity(2048);
    for path in paths {
        let mut subfiles = WalkBuilder::new(path)
            // we want to follow all symlinks in case of complex dir structures
            .follow_links(true)
            .add_custom_ignore_filename(".plexignore")
            .build()
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
            CombinedExtractor::from_str(&filename),
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
pub trait MediaMatcher: Send + Sync {
    async fn batch_match(
        &self,
        tx: &mut dim_database::Transaction<'_>,
        provider: Arc<dyn ExternalQueryIntoShow>,
        work: Vec<WorkUnit>,
    ) -> Result<(), Error>;

    /// Match a WorkUnit to a specific external id.
    async fn match_to_id(
        &self,
        tx: &mut dim_database::Transaction<'_>,
        provider: Arc<dyn ExternalQueryIntoShow>,
        work: WorkUnit,
        external_id: &str,
    ) -> Result<(), Error>;
}

pub async fn insert_mediafiles(
    conn: &mut dim_database::DbConnection,
    library_id: i64,
    dirs: Vec<impl AsRef<Path> + Send + 'static>,
) -> Result<Vec<WorkUnit>, Error> {
    let now = Instant::now();
    let subfiles = tokio::task::spawn_blocking(|| get_subfiles(dirs.into_iter()))
        .await
        .unwrap();
    let elapsed = now.elapsed();

    info!(
        elapsed_ms = elapsed.as_millis(),
        files = subfiles.len(),
        "Walked all target directories."
    );

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

    for chunk in insertables.chunks(256) {
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
    conn: &mut dim_database::DbConnection,
    library_id: i64,
    dirs: Vec<impl AsRef<Path> + Send + 'static>,
    tx: EventTx,
    media_type: MediaType,
    provider: Arc<dyn ExternalQueryIntoShow>,
) -> Result<(), Error> {
    info!(library_id, "Scanning library");

    tx.send(
        dim_events::Message {
            id: library_id,
            event_type: dim_events::PushEventType::EventStartedScanning,
        }
        .to_string(),
    )
    .map_err(|x| Error::EventDispatch(x.into()))?;

    let matcher = match media_type {
        MediaType::Movie => Arc::new(movie::MovieMatcher) as Arc<dyn MediaMatcher>,
        MediaType::Tv => Arc::new(tv_show::TvMatcher) as Arc<dyn MediaMatcher>,
        _ => unimplemented!(),
    };

    let mut lock = conn.writer().lock_owned().await;
    let mut db_tx = dim_database::write_tx(&mut lock)
        .await
        .map_err(|e| Error::DatabaseError(e.into()))?;

    MediaFile::delete_by_lib_id(&mut db_tx, library_id)
        .await
        .map_err(|e| Error::DatabaseError(e.into()))?;

    db_tx
        .commit()
        .await
        .map_err(|e| Error::DatabaseError(e.into()))?;

    let now = Instant::now();
    let workunits = insert_mediafiles(conn, library_id, dirs).await?;
    let workunits_size = workunits.len();

    info!(
        library_id,
        units = workunits_size,
        elapsed_ms = now.elapsed().as_millis(),
        "Walked and inserted mediafiles."
    );

    // NOTE: itertools::GroupBy is used across an await point and thus must also be Sync. This
    // breaks some of our higher-level logic where we spawn this task. Thus we collect it before
    // we proceed consuming it.
    let chunk_iter = workunits
        .into_iter()
        .chunks(128)
        .into_iter()
        .map(|x| x.collect())
        .collect::<Vec<_>>();

    // TODO: We can receive work over a channel so that we can in parallel create new mediafiles
    // and match objects.
    for unit in chunk_iter.into_iter() {
        let mut lock = conn.writer().lock_owned().await;
        let mut tx = dim_database::write_tx(&mut lock)
            .await
            .map_err(|e| Error::DatabaseError(e.into()))?;

        if let Err(e) = matcher.batch_match(&mut tx, provider.clone(), unit).await {
            error!(error = ?e, "Failed to match batch of mediafiles.");
        }

        tx.commit()
            .await
            .map_err(|e| Error::DatabaseError(e.into()))?;
    }

    info!(
        library_id,
        units = workunits_size,
        elapsed_ms = now.elapsed().as_millis(),
        "Finished scanning library."
    );

    tx.send(
        dim_events::Message {
            id: library_id,
            event_type: dim_events::PushEventType::EventStoppedScanning,
        }
        .to_string(),
    )
    .map_err(|e| Error::EventDispatch(e.into()))?;

    Ok(())
}

pub async fn start(
    conn: &mut dim_database::DbConnection,
    library_id: i64,
    tx: EventTx,
    provider: Arc<dyn ExternalQueryIntoShow>,
) -> Result<(), Error> {
    let mut tx_ = conn
        .read()
        .begin()
        .await
        .map_err(|e| Error::DatabaseError(e.into()))?;

    let lib = Library::get_one(&mut tx_, library_id)
        .await
        .map_err(|e| Error::LibraryNotFound(e))?;

    start_custom(
        conn,
        library_id,
        lib.locations,
        tx,
        lib.media_type,
        provider,
    )
    .await
}

/// Function formats the path where assets are stored.
pub fn format_path(x: Option<String>) -> String {
    x.map(|x| format!("images/{}", x.trim_start_matches('/')))
        .unwrap_or_default()
}
