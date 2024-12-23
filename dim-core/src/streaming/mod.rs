pub mod ffprobe;

use cfg_if::cfg_if;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::utils::ffpath;

lazy_static::lazy_static! {
    pub static ref STREAMING_SESSION: Arc<RwLock<HashMap<String, HashMap<String, String>>>> = Arc::new(RwLock::new(HashMap::new()));
    pub static ref FFMPEG_BIN: &'static str = Box::leak(ffpath("utils/ffmpeg").into_boxed_str());
    pub static ref FFPROBE_BIN: &'static str = {
        cfg_if! {
            if #[cfg(test)] {
                "/usr/bin/ffprobe"
            } else if #[cfg(bench)] {
                "/usr/bin/ffprobe"
            } else {
                Box::leak(ffpath("utils/ffprobe").into_boxed_str())
            }
        }
    };
}

use std::process::Command;

/// ffcheck - Check if "ffmpeg" and "ffprobe" are accessable through `std::process::Command`.
///
/// This will run `ffmpeg -version` and `ffprobe -version` and return a vec of the stdout
/// output if successfull or the binaries name if not.
///
/// # Example
///
/// ```ignore
/// use streaming::ffcheck;
///
/// for result in ffcheck() {
///     match result {
///         Ok(stdout) => println!("{:?}", stdout),
///         Err(program) => eprintln!("Failed to get the `-version` output of {:?}", program),
///     }
/// }
/// ```
pub fn ffcheck() -> Vec<Result<Box<str>, &'static str>> {
    let mut results = vec![];

    for program in [*FFMPEG_BIN, *FFPROBE_BIN].iter() {
        if let Ok(output) = Command::new(program).arg("-version").output() {
            let stdout = String::from_utf8(output.stdout)
                .expect("Failed to decode subprocess stdout.")
                .into_boxed_str();

            results.push(Ok(stdout));
        } else {
            results.push(Err(*program));
        }
    }

    results
}

#[derive(Clone, Copy)]
pub struct Quality {
    pub height: u64,
    pub bitrate: u64,
}

pub fn get_qualities(_height: u64, _bitrate: u64) -> Vec<&'static Quality> {
    VIDEO_QUALITIES.iter().collect()
}

pub const VIDEO_QUALITIES: [Quality; 3] = [
    Quality {
        height: 1080,
        bitrate: 10_000_000,
    },
    Quality {
        height: 720,
        bitrate: 5_000_000,
    },
    Quality {
        height: 480,
        bitrate: 1_000_000,
    },
];

#[derive(Clone)]
pub struct Avc1Level {
    pub level: u64,
    pub macro_blocks_rate: u64,
    pub max_frame_size: u64,
    pub max_bitrate: u64,
}

impl ToString for Avc1Level {
    fn to_string(&self) -> String {
        format!("avc1.6400{:x}", self.level)
    }
}

pub fn level_to_tag(level: i64) -> Option<Avc1Level> {
    let level = level as u64;
    AVC1_LEVELS.iter().find(|&x| x.level == level).cloned()
}

pub fn get_avc1_tag(width: u64, height: u64, bitrate: u64, framerate: f64) -> Avc1Level {
    let macro_blocks = (width as f64 / 16.0) * (height as f64 / 16.0);
    let blocks_per_sec = macro_blocks * framerate;

    let mut avc1_levels = AVC1_LEVELS.iter().filter(|&x| {
        x.max_bitrate > bitrate
            && (macro_blocks as u64) < x.max_frame_size
            && blocks_per_sec < x.macro_blocks_rate as f64
    });

    avc1_levels.next().cloned().unwrap()
}

pub const AVC1_LEVELS: [Avc1Level; 20] = [
    Avc1Level {
        level: 9,
        macro_blocks_rate: 1_485,
        max_frame_size: 99,
        max_bitrate: 128_000,
    },
    Avc1Level {
        level: 10,
        macro_blocks_rate: 1_485,
        max_frame_size: 99,
        max_bitrate: 64_000,
    },
    Avc1Level {
        level: 11,
        macro_blocks_rate: 3_000,
        max_frame_size: 396,
        max_bitrate: 192_000,
    },
    Avc1Level {
        level: 12,
        macro_blocks_rate: 6_000,
        max_frame_size: 396,
        max_bitrate: 384_000,
    },
    Avc1Level {
        level: 13,
        macro_blocks_rate: 11_880,
        max_frame_size: 396,
        max_bitrate: 768_000,
    },
    Avc1Level {
        level: 20,
        macro_blocks_rate: 11_880,
        max_frame_size: 396,
        max_bitrate: 2_000_000,
    },
    Avc1Level {
        level: 21,
        macro_blocks_rate: 19_800,
        max_frame_size: 792,
        max_bitrate: 4_000_000,
    },
    Avc1Level {
        level: 22,
        macro_blocks_rate: 20_250,
        max_frame_size: 1_620,
        max_bitrate: 4_000_000,
    },
    Avc1Level {
        level: 30,
        macro_blocks_rate: 40_500,
        max_frame_size: 1_620,
        max_bitrate: 10_000_000,
    },
    Avc1Level {
        level: 31,
        macro_blocks_rate: 108_000,
        max_frame_size: 3600,
        max_bitrate: 14_000_000,
    },
    Avc1Level {
        level: 32,
        macro_blocks_rate: 216_000,
        max_frame_size: 5_120,
        max_bitrate: 20_000_000,
    },
    Avc1Level {
        level: 40,
        macro_blocks_rate: 245_760,
        max_frame_size: 8_192,
        max_bitrate: 20_000_000,
    },
    Avc1Level {
        level: 41,
        macro_blocks_rate: 245_760,
        max_frame_size: 8_192,
        max_bitrate: 50_000_000,
    },
    Avc1Level {
        level: 42,
        macro_blocks_rate: 522_240,
        max_frame_size: 8_704,
        max_bitrate: 50_000_000,
    },
    Avc1Level {
        level: 50,
        macro_blocks_rate: 589_824,
        max_frame_size: 22_080,
        max_bitrate: 135_000_000,
    },
    Avc1Level {
        level: 51,
        macro_blocks_rate: 983_040,
        max_frame_size: 36_864,
        max_bitrate: 240_000_000,
    },
    Avc1Level {
        level: 52,
        macro_blocks_rate: 2_073_600,
        max_frame_size: 36_864,
        max_bitrate: 240_000_000,
    },
    Avc1Level {
        level: 60,
        macro_blocks_rate: 4_177_920,
        max_frame_size: 139_264,
        max_bitrate: 240_000_000,
    },
    Avc1Level {
        level: 61,
        macro_blocks_rate: 8_355_840,
        max_frame_size: 139_264,
        max_bitrate: 480_000_000,
    },
    Avc1Level {
        level: 62,
        macro_blocks_rate: 16_711_680,
        max_frame_size: 139_264,
        max_bitrate: 800_000_000,
    },
];
