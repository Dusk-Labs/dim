use crate::AppState;
use axum::body;
use axum::body::Empty;
use axum::body::Full;
use axum::extract::State;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::Uri;
use axum::http::Request;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

use dim_core::errors;
use dim_core::fetcher::insert_into_queue;
use dim_database::asset;

use http::header;
use http::StatusCode;
use rust_embed::RustEmbed;

use std::path;
use std::path::PathBuf;

use serde::Deserialize;

cfg_if::cfg_if! {
    if #[cfg(feature = "embed_ui")] {

        #[derive(RustEmbed)]
        #[folder = "../ui/public/"]
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

pub async fn react_routes() -> Result<impl IntoResponse, errors::DimError> {
    if let Some(x) = Asset::get("/index.html") {
        Ok(Html(x.data.into_owned()).into_response())
    } else {
        Err(errors::DimError::NotFoundError)
    }
}

#[derive(Deserialize)]
pub struct ImageParams {
    _w: Option<u32>,
    _h: Option<u32>,
    #[serde(default)]
    attach_accents: bool,
}

pub async fn get_image(
    State(AppState { conn, .. }): State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ImageParams>,
) -> Result<impl IntoResponse, errors::DimError> {
    let meta_path = dim_core::core::METADATA_PATH.get().unwrap();
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
    if !path::Path::new(&file_path).exists() {
        if let Ok(x) = asset::Asset::get_url_by_file(&mut tx, &url_path).await {
            insert_into_queue(x, path.as_str().into(), true).await;
        }

        return Err(errors::DimError::NotFoundError);
    }

    let image = tokio::fs::read(file_path).await.ok();

    let accents = match (image.as_ref(), params.attach_accents) {
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
        let mut resp = Response::builder()
            .status(StatusCode::OK)
            .header("ContentType", "image/jpeg");

        if let Some(accents) = accents {
            resp = resp.header("X-IMAGE-ACCENTS", accents);
        }

        return Ok(resp.body(body::boxed(Full::from(data))).map_err(|_| errors::DimError::NotFoundError));
    }

    Err(errors::DimError::NotFoundError)
}

pub async fn dist_static<T>(
    uri: Uri,
    req: Request<T>,
) -> Result<impl IntoResponse, errors::DimError> {
    let path = PathBuf::from(uri.path());
    if let Some(content) = Asset::get(path.to_str().unwrap()) {
        let hash = hex::encode(content.metadata.sha256_hash());
        if req
          .headers()
          .get(header::IF_NONE_MATCH)
          .map(|etag| etag.to_str().unwrap_or("000000").eq(&hash))
          .unwrap_or(false)
        {
            return Ok(Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .body(body::boxed(Empty::new()))
                .unwrap());
        }
        let mime = match path.extension().and_then(|x| x.to_str()) {
            Some("js") => "application/javascript",
            Some("map") => "application/json",
            Some("css") => "text/css",
            Some("woff") => "font/woff",
            Some("woff2") => "font/woff2",
            Some("png") => "image/png",
            Some("svg") => "image/svg+xml",
            Some("ttf") => "font/ttf",
            Some("json") => "application/json",
            Some("wasm") => "application/wasm",
            Some("data") => "application/octet-stream",
            _ => return Err(errors::DimError::NotFoundError),
        };

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", mime)
            .header(header::ETAG, hash)
            .body(body::boxed(Full::from(content.data.into_owned())))
            .unwrap())
    } else {
        Err(errors::DimError::NotFoundError)
    }
}
