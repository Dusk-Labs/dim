extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate dim_database;
extern crate uuid;

pub mod ffprobe;
pub mod ffmpeg;

pub static FFMPEG_BIN: &'static str = "/usr/bin/ffmpeg";
pub static FFPROBE_BIN: &'static str = "/usr/bin/ffprobe";
