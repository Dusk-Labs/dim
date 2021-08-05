use rust_embed::RustEmbed;
use warp::Filter;
use warp::Reply;
use warp::path;
use http::StatusCode;

pub fn statik_routes(
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::dist_static()
        .or(filters::get_image())
        .or(filters::react_routes())
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use std::path::PathBuf;
    use serde::Deserialize;
    use warp::Filter;
    use rust_embed::RustEmbed;
    use super::super::global_filters::with_state;

    pub fn react_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::get().and_then(super::react_routes)
    }

    pub fn get_image() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct QueryArgs {
            w: Option<u32>,
            h: Option<u32>,
        }

        let metadata_path = crate::core::METADATA_PATH.get().unwrap();

        warp::path!("images" / ..)
            .and(warp::get())
            .and(warp::path::tail())
            .and(warp::query::query::<QueryArgs>())
            .and(with_state(metadata_path.clone()))
            .and_then(|x: warp::path::Tail, QueryArgs { w, h }: QueryArgs, meta_path: String| async move {
                super::get_image(x, w, h, meta_path).await
            })
    }

    pub fn dist_static() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("static").and(warp::path::full()).and_then(
            |x: warp::path::FullPath| async move {
                if let Some(y) = super::Asset::get(x.as_str()) {
                    let path = PathBuf::from(x.as_str());
                    let mime = match path.extension().and_then(|x| x.to_str()) {
                        Some("js") => "application/javascript",
                        Some("css") => "text/css",
                        Some("woff2") => "font/woff2",
                        Some("png") => "image/png",
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
    resize_w: Option<u32>,
    resize_h: Option<u32>,
    meta_path: String
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut file_path = PathBuf::from(&meta_path);
    file_path.push(path.as_str());

    /*
    let image = if let (Some(w), Some(h)) = (resize_w, resize_h) {
        spawn_blocking(move ||  { resize_image(file_path, w, h).ok() }).await.unwrap()
    } else {
        tokio::fs::read(file_path).await.ok()
    };
    */

    let image = tokio::fs::read(file_path).await.ok();

    if let Some(data) = image {
        return warp::http::Response::builder()
            .status(StatusCode::OK)
            .header("ContentType", "image/jpeg")
            .body(data)
            .map_err(|_| warp::reject::not_found());
    }

    Err(warp::reject::not_found())
}
