use crate::core::DbConnection;
use crate::errors;
use crate::routes::construct_standard;

use auth::Wrapper as Auth;
use cfg_if::cfg_if;
use database::{
    episode::Episode,
    genre::*,
    library::MediaType,
    media::Media,
    mediafile::MediaFile,
    progress::Progress,
    schema::{genre_media, media, season},
    season::Season,
};
use diesel::prelude::*;
use diesel::sql_types::Text;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

use std::fs;
use std::io;
use std::path::PathBuf;

// Necessary to emulate ilike.
sql_function!(fn upper(x: Text) -> Text);

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
        .map(|x| x.path().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    dirs.sort();
    Ok(dirs)
}

#[get("/filebrowser")]
pub fn get_root_directory_structure(_user: Auth) -> Result<Json<Vec<String>>, errors::DimError> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            Ok(Json(enumerate_directory(r"C:\")?))
        } else {
            Ok(Json(enumerate_directory("/")?))
        }
    }
}

#[get("/filebrowser/<path..>")]
pub fn get_directory_structure(
    path: Option<PathBuf>,
    _user: Auth,
) -> Result<Json<Vec<String>>, errors::DimError> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let path_prefix = r"C:\";
        } else {
            let path_prefix = "/";
        }
    }

    let path = if let Some(path) = path {
        if path.starts_with(path_prefix) {
            path
        } else {
            let mut new_path = PathBuf::new();
            new_path.push(path_prefix);
            new_path.push(path);
            new_path
        }
    } else {
        path_prefix.into()
    };

    Ok(Json(enumerate_directory(path)?))
}

#[get("/search?<query>&<year>&<library_id>&<genre>&<quick>")]
pub fn search(
    conn: DbConnection,
    query: Option<&RawStr>,
    year: Option<i32>,
    library_id: Option<i32>,
    genre: Option<String>,
    quick: Option<bool>,
    user: Auth,
) -> Result<Json<Vec<JsonValue>>, errors::DimError> {
    let quick = quick.unwrap_or(false);
    let mut result = media::table.into_boxed();

    result = result.filter(media::media_type.ne(MediaType::Episode));

    if let Some(query_string) = query {
        let query_string = query_string
            .url_decode_lossy()
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
            .join("% %");

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                result = result.filter(media::name.ilike(format!("%{}%", query_string)));
            } else {
                result = result.filter(upper(media::name).like(format!("%{}%", query_string)));
            }
        }
    }

    if let Some(x) = year {
        result = result.filter(media::year.eq(x));
    }

    if let Some(x) = library_id {
        result = result.filter(media::library_id.eq(x));
    }

    if let Some(x) = genre {
        let genre_row = Genre::get_by_name(conn.as_ref(), x)?.id;

        let new_result = result
            .inner_join(genre_media::table)
            .filter(genre_media::genre_id.eq(genre_row));

        let new_result = new_result.load::<Media>(conn.as_ref())?;
        return Ok(Json(
            new_result
                .iter()
                .filter_map(|x| construct_standard(&conn, x, &user, quick).ok())
                .collect::<Vec<JsonValue>>(),
        ));
    }

    let result = result.load::<Media>(conn.as_ref()).unwrap_or_default();
    Ok(Json(
        result
            .iter()
            .filter_map(|x| construct_standard(&conn, x, &user, quick).ok())
            .collect::<Vec<JsonValue>>(),
    ))
}
