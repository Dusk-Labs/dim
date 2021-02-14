pub mod ffprobe;
pub mod transcode;

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        macro_rules! which {
            ($prog:expr) => {$prog.into()};
        }
    } else {
        macro_rules! which {
            ($prog:expr) => {
                String::from_utf8(Command::new("which").arg($prog).output().unwrap().stdout)
                    .expect("Failed to decode `which $prog`.")
                    .trim_end()
                    .into();
            };
        }
    }
}

lazy_static! {
    pub static ref FFMPEG_BIN: Box<str> = which!("./utils/ffmpeg");
    pub static ref FFPROBE_BIN: Box<str> = which!("./utils/ffprobe");
    pub static ref STREAMING_SESSION: Arc<RwLock<HashMap<String, HashMap<String, String>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

use std::process::Command;

/// ffcheck - Check if "ffmpeg" and "ffprobe" are accessable through `std::process::Command`.
///
/// This will run `ffmpeg -version` and `ffprobe -version` and push their stdout's
/// onto the provided `bucket`.
///
/// # Arguments
/// * `bucket` - a `Vec<Box<str>>` to push the commands stdout's onto
///
/// # Example
/// ```
/// use streamer::ffcheck;
///
/// let mut bucket: Vec<Box<str>> = Vec::new();
/// if let Err(why) = ffcheck(&mut bucket) {
///     eprintln!("Could not find: {}", why);
///     std::process::exit(1);
/// }
///
/// for item in bucket.iter() {
///     println!("\n{}", item);
/// }    
/// ```
pub fn ffcheck<'a>(bucket: &'a mut Vec<Box<str>>) -> Result<(), Box<&str>> {
    for program in ["./utils/ffmpeg", "./utils/ffprobe"].iter() {
        if let Ok(output) = Command::new(program).arg("-version").output() {
            let stdout = String::from_utf8(output.stdout)
                .expect("Failed to decode subprocess stdout.")
                .into_boxed_str();

            bucket.push(stdout);
        } else {
            return Err(Box::new(program));
        }
    }

    Ok(())
}
