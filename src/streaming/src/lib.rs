extern crate database;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod ffmpeg;
pub mod ffprobe;

pub static FFMPEG_BIN: &str = "/usr/bin/ffmpeg";
pub static FFPROBE_BIN: &str = "/usr/bin/ffprobe";
