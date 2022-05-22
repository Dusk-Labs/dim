//! Module contains all the code for the new generation media scanner.

mod mediafile;
mod movie;
#[cfg(test)]
mod tests;

use anitomy::Anitomy;
use async_trait::async_trait;

use database::mediafile::MediaFile;

use super::external::filename::FilenameMetadata;
use super::external::filename::Metadata;
use super::external::ExternalQuery;

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use torrent_name_parser::Metadata as TorrentMetadata;
use tracing::warn;
use walkdir::WalkDir;

pub(super) static SUPPORTED_EXTS: &[&str] = &["mp4", "mkv", "avi", "webm"];

/// Function recursively walks the paths passed and returns all files in those directories.
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
    async fn batch_match(self: Arc<Self>, provider: Arc<dyn ExternalQuery>, work: Vec<WorkUnit>);
}
