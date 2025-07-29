mod file_walker;
pub(crate) mod mediafile;

use std::fs::hard_link;
use std::fs::File;
use std::path::PathBuf;

use tempfile::TempDir;

const TEST_MP4_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/scanner/tests/data/test.mp4"
);

pub fn temp_dir<'a>(files: impl IntoIterator<Item = &'a str>) -> tempfile::TempDir {
    let tempdir = tempfile::Builder::new()
        .prefix("tmp")
        .tempdir()
        .expect("Failed to create temporary directory for tests.");

    for file in files.into_iter() {
        let file_path = tempdir.path().join(file);

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create parent dir");
        }

        File::create(file_path).expect("Failed to create test files for testing.");
    }

    tempdir
}

#[track_caller]
pub fn temp_dir_symlink<'a>(
    files: impl Iterator<Item = impl AsRef<str>>,
    target_file: &'a str,
) -> (TempDir, Vec<PathBuf>) {
    let tempdir = tempfile::Builder::new()
        .prefix("tmp")
        .tempdir()
        .expect("Failed to create temporary directory for tests.");

    let new_target_file = tempdir.path().join("target");

    std::fs::copy(target_file, &new_target_file).expect("failed copying target file to tempdir");

    let target_file = new_target_file;

    let mut absolute = vec![];

    for file in files.into_iter() {
        let file_path = tempdir.path().join(file.as_ref());

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create parent dir");
        }

        hard_link(&target_file, &file_path).expect("Failed to create hard link to test file.");

        absolute.push(file_path);
    }

    (tempdir, absolute)
}
