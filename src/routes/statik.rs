use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;

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
pub fn dist_static<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    let file_path = format!("static/{}", filename);

    dist_file(&file_path)
}

#[get("/<file>", rank = 1)]
pub fn dist_asset<'r>(file: String) -> response::Result<'r> {
    dist_file(file.as_ref())
}

pub fn dist_file<'r>(file: &str) -> response::Result<'r> {
    Asset::get(file).map_or_else(index_redirect, |d| {
        let ext = Path::new(file)
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| Status::new(400, "Could not get file extension"))?;

        let content_type = ContentType::from_extension(ext)
            .ok_or_else(|| Status::new(400, "Could not get file content type"))?;

        response::Response::build()
            .header(content_type)
            .sized_body(Cursor::new(d))
            .ok()
    })
}

#[get("/")]
pub fn index_redirect<'r>() -> response::Result<'r> {
    Asset::get("index.html").map_or_else(
        || Err(Status::NotFound),
        |x| {
            response::Response::build()
                .header(ContentType::HTML)
                .sized_body(Cursor::new(x))
                .ok()
        },
    )
}

#[get("/images/<file..>", rank = 1)]
pub fn get_image<'r>(file: PathBuf) -> response::Result<'r> {
    let mut pathbuf = PathBuf::from(crate::core::METADATA_PATH.get().unwrap());
    pathbuf.push(file);

    File::open(pathbuf).map_or_else(
        |_| Err(Status::NotFound),
        |x| {
            response::Response::build()
                .header(ContentType::JPEG)
                .sized_body(x)
                .ok()
        },
    )
}

#[get("/<path..>", rank = 4)]
pub fn react_routes<'r>(path: PathBuf) -> response::Result<'r> {
    Asset::get("index.html").map_or_else(
        || Err(Status::NotFound),
        |x| {
            response::Response::build()
                .header(ContentType::HTML)
                .sized_body(Cursor::new(x))
                .ok()
        },
    )
}
