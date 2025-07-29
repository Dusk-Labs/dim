use super::temp_dir;
use std::path::PathBuf;

#[tokio::test(flavor = "multi_thread")]
async fn test_walkdir() {
    let tempdir = temp_dir(vec![
        "file1.mkv",
        "file2.avi",
        "file3.txt",
        "a/b/file4.webm",
        "a/file5.mp4",
        ".hidden.mp4",
    ]);

    let mut files = super::super::get_subfiles([tempdir.path()].iter());
    files.sort();

    let mut expected: Vec<PathBuf> =
        IntoIterator::into_iter(["file1.mkv", "file2.avi", "a/b/file4.webm", "a/file5.mp4"])
            .map(|x| tempdir.path().join(x))
            .collect();

    expected.sort();

    assert_eq!(files, expected);
}
