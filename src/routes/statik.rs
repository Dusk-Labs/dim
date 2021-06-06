use std::ffi::OsStr;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use warp::Filter;
use warp::Reply;

pub fn statik_routes(
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::dist_static()
        .or(filters::get_image())
        .or(filters::react_routes())
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use warp::reject;
    use warp::Filter;
    use warp::Reply;

    use std::path::PathBuf;

    pub fn react_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::get().and_then(super::react_routes)
    }

    pub fn get_image() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let metadata_path = crate::core::METADATA_PATH.get().unwrap();
        warp::path("images").and(warp::fs::dir(metadata_path))
    }

    pub fn dist_static() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("static").and(warp::path::full()).and_then(
            |x: warp::path::FullPath| async move {
                // skip the first char because its always a `/`
                if let Some(y) = super::Asset::get(&format!("./{}", x.as_str())) {
                    let path = PathBuf::from(x.as_str());
                    let mime = match path.extension().and_then(|x| x.to_str()) {
                        Some("js") => "application/javascript",
                        Some("css") => "text/css",
                        Some("woff2") => "font/woff2",
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
}

#[derive(RustEmbed)]
#[cfg_attr(any(feature = "embed_ui", target_os = "windows"), folder = "ui/build/")]
#[cfg_attr(
    all(not(feature = "embed_ui"), not(target_os = "windows")),
    folder = "/dev/null"
)]
pub(self) struct Asset;

pub async fn react_routes() -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(x) = Asset::get("index.html") {
        Ok(warp::reply::html(x.into_owned()).into_response())
    } else {
        Err(warp::reject::not_found())
    }
}
