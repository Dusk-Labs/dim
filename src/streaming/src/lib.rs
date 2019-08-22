extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub mod ffprobe;

pub static FFMPEG_BIN: &'static str = "/usr/bin/ffmpeg";
pub static FFPROBE_BIN: &'static str = "/usr/bin/ffprobe";
