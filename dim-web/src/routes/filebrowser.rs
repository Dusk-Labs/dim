use axum::response::IntoResponse;
use axum::response::Response;
use axum::extract::Path;

use http::StatusCode;

use tokio::task::spawn_blocking;

use std::fs;
use std::io;
use std::path::PathBuf;

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum AuthError {
    /// IO Error.
    IOError,
    /// Not logged in.
    InvalidCredentials,
}

impl From<std::io::Error> for AuthError {
    fn from(_: std::io::Error) -> Self {
        Self::IOError
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            Self::IOError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}

pub fn enumerate_directory<T: AsRef<std::path::Path>>(path: T) -> io::Result<Vec<String>> {
    let mut dirs: Vec<String> = fs::read_dir(path)?
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| {
            !x.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
                && !x.path().is_file()
        })
        .map(|x| {
            let path = x.path().to_string_lossy().to_string().replace("\\", "/");
            if cfg!(windows) {
                path.replace("C:", "")
            } else {
                path
            }
        })
        .collect::<Vec<_>>();

    dirs.sort();
    Ok(dirs)
}

pub async fn get_directory_structure(
    path: Option<Path<String>>,
) -> Result<axum::response::Response, AuthError> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let path_prefix = "C:/";
        } else {
            let path_prefix = "/";
        }
    }

    let path: PathBuf = match path {
        Some(Path(p)) => {
            let path = if p.starts_with(path_prefix) {
                PathBuf::from(p)
            } else {
                let mut new_path = PathBuf::new();
                new_path.push(path_prefix);
                new_path.push(p);
                new_path
            };
            path
        }
        None => {
            PathBuf::from(path_prefix)
        }
    };

    Ok(axum::response::Json(
        &spawn_blocking(|| enumerate_directory(path))
            .await
            .unwrap()?,
    ).into_response())
}
