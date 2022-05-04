use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use database::library::{InsertableLibrary, MediaType};
use tempfile::TempDir;

fn create_file(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_owned();
    path.push(name);

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();

    path.canonicalize().unwrap()
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub path_as_string: String,
}

#[derive(Debug)]
pub struct LibDir {
    path: TempDir,
    pub name: String,
    pub media_type: MediaType,
    pub files: Vec<File>,
    pub lib_id: i64,
}

impl LibDir {
    pub fn new(name: String, media_type: MediaType) -> Self {
        let path = tempfile::Builder::new()
            .prefix("tmp")
            .tempdir()
            .expect("it should always be possible to create a tmpdir");

        Self {
            name,
            media_type,
            files: vec![],
            lib_id: 0,
            path,
        }
    }

    pub fn as_insertable_library(&self) -> InsertableLibrary {
        let Self {
            path,
            name,
            media_type,
            ..
        } = self;

        InsertableLibrary {
            name: format!("{name}"),
            locations: vec![path.path().to_owned().to_string_lossy().to_string()],
            media_type: *media_type,
        }
    }

    pub fn create_file(&mut self, file_name: &str) -> &File {
        let file = create_file(self.path.path(), file_name);
        let file_path = file.to_string_lossy().to_string();

        let file = File {
            path: file,
            path_as_string: file_path,
        };

        self.files.push(file.clone());

        self.files.last().unwrap()
    }
}
