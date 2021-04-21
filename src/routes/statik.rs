use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;
use rocket::response::NamedFile;

use std::ffi::OsStr;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[cfg_attr(any(feature = "embed_ui", target_os = "windows"), folder = "ui/build/")]
#[cfg_attr(
    all(not(feature = "embed_ui"), not(target_os = "windows")),
    folder = "/dev/null"
)]
struct Asset;

#[get("/static/<file..>")]
pub async fn dist_static<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    let file_path = format!("static/{}", filename);

    dist_file(&file_path).await
}

#[get("/<file>", rank = 1)]
pub async fn dist_asset<'r>(file: String) -> response::Result<'r> {
    dist_file(file.as_ref()).await
}

pub async fn dist_file<'r>(file: &str) -> response::Result<'r> {
    if let Some(x) = Asset::get(file) {
        let ext = Path::new(file)
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| Status::new(400, "Could not get file extension"))?;

        let content_type = ContentType::from_extension(ext)
            .ok_or_else(|| Status::new(400, "Could not get file content type"))?;

        response::Response::build()
            .header(content_type)
            .streamed_body(Cursor::new(x))
            .ok()
    } else {
        index_redirect().await
    }
}

#[get("/")]
pub async fn index_redirect<'r>() -> response::Result<'r> {
    if let Some(x) = Asset::get("index.html") {
        response::Response::build()
            .header(ContentType::HTML)
            .streamed_body(Cursor::new(x))
            .ok()
    } else {
        Err(Status::NotFound)
    }
}

#[get("/images/<file..>", rank = 3)]
pub async fn get_image<'r>(file: PathBuf) -> Option<NamedFile> {
    let mut pathbuf = PathBuf::from(crate::core::METADATA_PATH.get().unwrap());
    pathbuf.push(file);

    NamedFile::open(pathbuf).await.ok()
}

#[get("/<path..>", rank = 4)]
pub async fn react_routes<'r>(path: PathBuf) -> response::Result<'r> {
    if let Some(x) = Asset::get("index.html") {
        response::Response::build()
            .header(ContentType::HTML)
            .streamed_body(Cursor::new(x))
            .ok()
    } else {
        Err(Status::NotFound)
    }
}
