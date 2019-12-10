#![feature(result_map_or_else)]
extern crate database;
extern crate lazy_static;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod ffmpeg;
pub mod ffprobe;

use lazy_static::lazy_static;

macro_rules! which {
    ($prog:expr) => {
        String::from_utf8(Command::new("which").arg($prog).output().unwrap().stdout)
            .expect("Failed to decode `wich $prog`.")
            .into_boxed_str()
    };
}

lazy_static! {
    pub static ref FFMPEG_BIN: Box<str> = { which!("ffmpeg") };
    pub static ref FFPROBE_BIN: Box<str> = { which!("ffprobe") };
}

use std::process::Command;

pub fn ffcheck<'a>(bucket: &'a mut Vec<Box<str>>) -> Result<(), Box<&str>> {
    for program in ["ffmpeg", "ffprobe"].iter() {
        if let Ok(output) = Command::new(program).arg("-version").output() {
            let stdout = dbg!(String::from_utf8(output.stdout)
                .expect("Failed to decode subprocess stdout.")
                .into_boxed_str());

            bucket.push(stdout);
        } else {
            return Err(Box::new(program));
        }
    }

    Ok(())
}
