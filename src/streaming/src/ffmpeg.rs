use dim_database::{get_conn, mediafile::MediaFile};
use std::collections::HashMap;
use std::fs;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref STREAMS: Arc<Mutex<HashMap<String, Child>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct FFmpeg {
    bin: String,
    mediafile: MediaFile,
    uuid: String,
    out_dir: String,
}

impl FFmpeg {
    pub fn new(ffmpeg_bin: &'static str, mediafile: i32) -> Result<Self, ()> {
        let uuid = Uuid::new_v4();
        let conn = match get_conn() {
            Ok(x) => x,
            Err(err) => panic!("[FFMPEG] New panic'd with {:?}", err),
        };

        Ok(Self {
            bin: ffmpeg_bin.to_owned(),
            mediafile: match MediaFile::get_one(&conn, mediafile) {
                Ok(x) => x,
                Err(_) => return Err(()),
            },
            uuid: uuid.to_hyphenated().to_string(),
            out_dir: format!(
                "/home/hinach4n/media/media1/transcoding/{}",
                uuid.to_hyphenated().to_string()
            ),
        })
    }

    /// TODO: Add params to select codec out and in, seek, and further params.
    pub fn stream(&mut self) -> Result<String, ()> {
        let input = format!("file:{}", self.mediafile.target_file.clone().as_str());
        let manifest = format!("{}/index.m3u8", self.out_dir);
        let chunks = format!("{}/%d.ts", self.out_dir);

        let _ = fs::create_dir(self.out_dir.clone());

        let mut process = Command::new(self.bin.clone());
        process
            .args(&["-fflags", "+genpts", "-noaccurate_seek"])
            .args(&["-f", "matroska,webm", "-i", input.as_str()])
            .args(&["-map_metadata", "-1"])
            .args(&["-map_chapters", "-1"])
            .args(&["-threads", "0"])
            .args(&["-map", "0:0", "-map", "0:1"])
            .args(&["-c:v:0", "copy", "-bsf:v", "h264_mp4toannexb"])
            .args(&["-force_key_frames", "expr:gte(t,n_forced*6)"])
            .args(&["-copyts", "-vsync", "-1", "-c:a:0", "aac"])
            .args(&["-strict", "experimental", "-ac", "2", "-af", "volume=2"])
            .args(&["-f", "segment", "-max_delay", "5000000"])
            .args(&["-avoid_negative_ts", "disabled", "-start_at_zero"])
            .args(&["-segment_time", "6", "-individual_header_trailer", "0"])
            .args(&["-break_non_keyframes", "1", "-segment_format", "mpegts"])
            .args(&["-segment_list_type", "m3u8", "-segment_start_number", "0"])
            .args(&["-segment_list", manifest.as_str(), "-y", chunks.as_str()]);

        let process = match process.spawn() {
            Ok(pid) => pid,
            Err(_) => return Err(()),
        };

        {
            STREAMS.lock().unwrap().insert(self.uuid.clone(), process);
        }

        Ok(self.uuid.clone())
    }

    pub fn stop(uuid: String) -> Result<(), std::io::Error> {
        if let Some(mut proc) = STREAMS.lock().unwrap().remove(&uuid) {
            return proc.kill();
        }

        Ok(())
    }
}
