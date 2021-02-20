use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;

use std::ffi::OsStr;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[cfg_attr(feature = "embed_ui", folder = "web_ui/build/")]
#[cfg_attr(not(feature = "embed_ui"), folder = "/dev/null")]
struct Asset;

#[get("/", rank = 1)]
pub fn index<'r>() -> response::Result<'r> {
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

#[get("/<file..>", rank = 2)]
pub fn dist_file<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    Asset::get(&filename).map_or_else(index, |d| {
        let ext = file
            .as_path()
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
