use dim_database::asset;
use http::StatusCode;
use rust_embed::RustEmbed;
use warp::path;
use warp::Reply;

use std::path::Path;
use std::path::PathBuf;

use crate::errors;
use crate::fetcher::insert_into_queue;

pub mod filters {
    use super::super::global_filters::with_state;
    #[allow(unused_imports)]
    use rust_embed::RustEmbed;
    use serde::Deserialize;
    use std::path::PathBuf;
    use warp::reject;
    use warp::Filter;

    pub fn react_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::get().and_then(super::react_routes)
    }

    pub fn get_image(
        conn: dim_database::DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct QueryArgs {
            w: Option<u32>,
            h: Option<u32>,
            #[serde(default)]
            attach_accents: bool,
        }

        let metadata_path = crate::core::METADATA_PATH.get().unwrap();

        warp::path!("images" / ..)
            .and(warp::get())
            .and(warp::path::tail())
            .and(warp::query::query::<QueryArgs>())
            .and(with_state(metadata_path.clone()))
            .and(with_state(conn))
            .and_then(
                |x,
                 QueryArgs {
                     w,
                     h,
                     attach_accents,
                 }: QueryArgs,
                 meta_path,
                 conn| async move {
                    super::get_image(x, w, h, meta_path, conn, attach_accents)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn dist_static() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("static").and(warp::path::full()).and_then(
            |x: warp::path::FullPath| async move {
                if let Some(y) = super::Asset::get(x.as_str()) {
                    let path = PathBuf::from(x.as_str());
                    let mime = match path.extension().and_then(|x| x.to_str()) {
                        Some("js") => "application/javascript",
                        Some("map") => "application/json",
                        Some("css") => "text/css",
                        Some("woff2") => "font/woff2",
                        Some("png") => "image/png",
                        Some("ttf") => "font/ttf",
                        Some("json") => "application/json",
                        Some("wasm") => "application/wasm",
                        Some("data") => "application/octet-stream",
                        _ => return Err(warp::reject::not_found()),
                    };

                    Ok(warp::http::response::Response::builder()
                        .status(200)
                        .header("Content-Type", mime)
                        .body(y.into_owned())
                        .unwrap())
                } else {
                    Err(warp::reject::not_found())
                }
            },
        )
    }

    pub fn ui_manifest() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("static" / "manifest.json").and_then(|| async {
            if let Some(resp) = super::Asset::get("static/manifest.json") {
                Ok(warp::http::response::Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(resp.into_owned())
                    .unwrap())
            } else {
                Err(warp::reject::not_found())
            }
        })
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "embed_ui")] {

        #[derive(RustEmbed)]
        #[folder = "../ui/build/"]
        #[prefix = "/"]
        pub(self) struct Asset;
    } else {
        use rust_embed::Filenames;
        use std::borrow::Cow;

        pub(self) struct Asset;

        impl RustEmbed for Asset {
            fn get(_: &str) -> Option<Cow<'static, [u8]>> {
                None
            }

            fn iter() -> Filenames {
                unimplemented!()
            }
        }
    }
}

pub async fn react_routes() -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(x) = Asset::get("/index.html") {
        Ok(warp::reply::html(x.into_owned()).into_response())
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn get_image(
    path: path::Tail,
    _resize_w: Option<u32>,
    _resize_h: Option<u32>,
    meta_path: String,
    conn: dim_database::DbConnection,
    attach_accents: bool,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut file_path = PathBuf::from(&meta_path);
    file_path.push(path.as_str());

    let mut url_path = PathBuf::from("images/");
    url_path.push(path.as_str());

    /*
    let image = if let (Some(w), Some(h)) = (resize_w, resize_h) {
        spawn_blocking(move ||  { resize_image(file_path, w, h).ok() }).await.unwrap()
    } else {
        tokio::fs::read(file_path).await.ok()
    };
    */

    let mut tx = conn.read().begin().await?;
    // FIXME (val): return not yet available error here as a hint that in the future this URL will
    // return 200 OK.
    if !Path::new(&file_path).exists() {
        if let Ok(x) = asset::Asset::get_url_by_file(&mut tx, &url_path).await {
            insert_into_queue(x, path.as_str().into(), true).await;
        }

        return Err(errors::DimError::NotFoundError);
    }

    let image = tokio::fs::read(file_path).await.ok();

    let accents = match (image.as_ref(), attach_accents) {
        (Some(data), true) => {
            if let Ok(image) = image::load_from_memory(&data) {
                Some(
                    dominant_color::get_colors(image.as_bytes(), false)
                        .chunks_exact(3)
                        .map(|rgb| match rgb {
                            [r, g, b] => format!("#{r:02x}{g:02x}{b:02x}"),
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>()
                        .join(","),
                )
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some(data) = image {
        let mut resp = warp::http::Response::builder()
            .status(StatusCode::OK)
            .header("ContentType", "image/jpeg");

        if let Some(accents) = accents {
            resp = resp.header("X-IMAGE-ACCENTS", accents);
        }

        return resp.body(data).map_err(|_| errors::DimError::NotFoundError);
    }

    Err(errors::DimError::NotFoundError)
}
