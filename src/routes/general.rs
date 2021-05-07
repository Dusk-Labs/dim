use crate::core::DbConnection;
use crate::errors;
use crate::routes::construct_standard;
use crate::routes::construct_standard_quick;

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
use futures::stream;
use futures::StreamExt;
use tokio::task::spawn_blocking;
use tokio_diesel::*;

use rocket::http::RawStr;
use rocket::State;
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
pub async fn get_root_directory_structure(
    _user: Auth,
) -> Result<Json<Vec<String>>, errors::DimError> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            Ok(Json(spawn_blocking(|| enumerate_directory(r"C:\")).await.unwrap()?))
        } else {
            Ok(Json(spawn_blocking(|| enumerate_directory("/")).await.unwrap()?))
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
pub async fn search(
    conn: State<'_, DbConnection>,
    query: Option<String>,
    year: Option<i32>,
    library_id: Option<i32>,
    genre: Option<String>,
    quick: Option<bool>,
    user: Auth,
) -> Result<Json<Vec<JsonValue>>, errors::DimError> {
    /*
     * NOTE: Until tokio-diesel merges support for BoxedDsl we cant stack filters.
    let mut result = media::table.into_boxed();

    result = result.filter(media::media_type.ne(MediaType::Episode));

    if let Some(query_string) = query {
        let query_string = query_string
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
        let genre_row = Genre::get_by_name(&conn, x).await?.id;

        let new_result = result
            .inner_join(genre_media::table)
            .filter(genre_media::genre_id.eq(genre_row));

        let new_result = new_result.load_async::<Media>(&conn).await?;

        return Ok(Json(
            stream::iter(new_result)
                .filter_map(|x| async move {
                    if quick {
                        construct_standard_quick(&x).await.ok()
                    } else {
                        construct_standard(&conn, &x, &user).await.ok()
                    }
                })
                .collect::<Vec<JsonValue>>()
                .await,
        ));
    }

    // to avoid weird issue with boxed dsl not being send
    let result = result.load_async::<Media>(&conn).await.unwrap_or_default();
    Ok(Json(
        stream::iter(result)
            .filter_map(|x| async {
                if quick {
                    construct_standard_quick(&x).await.ok()
                } else {
                    construct_standard(&conn, &x, &user).await.ok()
                }
            })
            .collect::<Vec<JsonValue>>()
            .await,
    ))
    */
    let quick = quick.unwrap_or(false);

    if let Some(query_string) = query {
        let query_string = query_string
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
            .join("% %");

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                let result = media::table.filter(media::name.ilike(format!("%{}%", query_string)));
            } else {
                let result = media::table.filter(upper(media::name).like(format!("%{}%", query_string)));
            }
        }

        let result = result.load_async::<Media>(&conn).await.unwrap_or_default();
        let mut items = Vec::new();

        for x in result {
            if quick {
                if let Ok(x) = construct_standard_quick(&x).await {
                    items.push(x);
                }
            } else {
                if let Ok(x) = construct_standard(&conn, &x, &user).await {
                    items.push(x);
                }
            }
        }

        return Ok(Json(items));
    }

    if let Some(x) = genre {
        let genre_row = Genre::get_by_name(&conn, x).await?.id;

        let new_result = media::table
            .inner_join(genre_media::table)
            .filter(genre_media::genre_id.eq(genre_row))
            .select(media::all_columns);

        let new_result = new_result.load_async::<Media>(&conn).await?;
        let mut items = Vec::new();

        for x in new_result {
            if quick {
                if let Ok(x) = construct_standard_quick(&x).await {
                    items.push(x);
                }
            } else {
                if let Ok(x) = construct_standard(&conn, &x, &user).await {
                    items.push(x);
                }
            }
        }

        return Ok(Json(items));
    }

    if let Some(x) = year {
        let result = media::table
            .filter(media::year.eq(year))
            .load_async::<Media>(&conn)
            .await?;

        let mut items = Vec::new();

        for x in result {
            if quick {
                if let Ok(x) = construct_standard_quick(&x).await {
                    items.push(x);
                }
            } else {
                if let Ok(x) = construct_standard(&conn, &x, &user).await {
                    items.push(x);
                }
            }
        }

        return Ok(Json(items));
    }

    Err(errors::DimError::NotFoundError)
}
