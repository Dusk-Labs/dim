use crate::{errors, streaming::STREAMING_SESSION};
use std::{
    collections::HashMap,
    fs,
    io::{BufReader, Read},
    process::{Child, Command, Stdio},
};
use stoppable_thread::{self, SimpleAtomicBool, StoppableHandle};
use uuid::Uuid;

const CHUNK_SIZE: u64 = 5;
/// Represents how many chunks we encode before we require a timeout reset.
/// Basically if within MAX_CHUNKS_AHEAD we do not get a timeout reset we kill the stream.
/// This can be tuned
const MAX_CHUNKS_AHEAD: u64 = 15;

pub struct Session {
    pub process_id: String,
    process: StoppableHandle<()>,
    start_number: u64,
    stream_type: StreamType,
    last_chunk: u64,
}

struct TranscodeHandler {
    id: String,
    process: Child,
}

enum StreamType {
    Video,
    Audio,
}

pub enum Profile {
    Direct,
    High,
    Medium,
    Low,
    Audio,
}

impl Profile {
    fn to_params(&self) -> (Vec<&'static str>, &'static str) {
        match self {
            Self::Direct => (vec!["-c:0", "copy"], "direct"),
            Self::High => (
                vec![
                    "-c:0",
                    "libx264",
                    "-b:v",
                    "5M",
                    "-preset:0",
                    "veryfast",
                    "-vf",
                    "scale=1280:-2",
                ],
                "5000kb",
            ),
            Self::Medium => (
                vec![
                    "-c:0",
                    "libx264",
                    "-b:v",
                    "2M",
                    "-preset",
                    "ultrafast",
                    "-vf",
                    "scale=720:-2",
                ],
                "2000kb",
            ),
            Self::Low => (
                vec![
                    "-c:0",
                    "libx264",
                    "-b:v",
                    "1M",
                    "-preset",
                    "ultrafast",
                    "-vf",
                    "scale=480:-2",
                ],
                "1000kb",
            ),
            Self::Audio => (vec![], "120kb"),
        }
    }

    pub fn from_string<T: AsRef<str>>(profile: T) -> Result<Self, errors::StreamingErrors> {
        Ok(match profile.as_ref() {
            "direct" => Self::Direct,
            "5000kb" => Self::High,
            "2000kb" => Self::Medium,
            "1000kb" => Self::Low,
            _ => return Err(errors::StreamingErrors::InvalidProfile),
        })
    }
}

impl<'a> Session {
    /// Function returns a new transcoding session based on the params passed, it also
    /// automatically calculates the hls_start attribute for streams started at an offset
    pub fn new_video(
        file: String,
        profile: Profile,
        start_number: u64,
        outdir: String,
    ) -> Result<Self, errors::StreamingErrors> {
        let file = format!("file://{}", file);
        let profile_args = profile.to_params();

        let _ = fs::create_dir_all(format!("{}/video/{}", outdir, profile_args.1));

        let mut video_args =
            Self::build_video(string_to_static_str(file), start_number, profile_args.0);

        video_args.push("-hls_segment_filename");
        video_args.push(string_to_static_str(format!(
            "{}/video/{}/%d.m4s",
            outdir, profile_args.1
        )));
        video_args.push(string_to_static_str(format!(
            "{}/video/{}/playlist.m3u8",
            outdir, profile_args.1
        )));
        let mut video_process = Command::new(super::FFMPEG_BIN.as_ref());
        video_process
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .args(video_args.as_slice());

        println!("Starting video stream with args: {:?}", video_args);

        let process_id = Uuid::new_v4().to_hyphenated().to_string();

        let mut video_process = TranscodeHandler::new(process_id.clone(), video_process.spawn()?);

        Ok(Self {
            process: stoppable_thread::spawn(move |signal| video_process.handle(signal)),
            process_id,
            start_number,
            last_chunk: start_number,
            stream_type: StreamType::Video,
        })
    }
    /// Function returns a new transcoding session based on the params passed, it also
    /// automatically calculates the hls_start attribute for streams started at an offset
    pub fn new_audio(
        file: String,
        start_number: u64,
        outdir: String,
    ) -> Result<Self, errors::StreamingErrors> {
        let file = format!("file://{}", file);

        let _ = fs::create_dir_all(format!("{}/audio/120kb", outdir));
        let mut audio_args = Self::build_audio(string_to_static_str(file), start_number);

        audio_args.push("-hls_segment_filename");
        audio_args.push(string_to_static_str(format!(
            "{}/audio/120kb/%d.m4s",
            outdir
        )));
        audio_args.push(string_to_static_str(format!(
            "{}/audio/120kb/playlist.m3u8",
            outdir
        )));

        let mut audio_process = Command::new(super::FFMPEG_BIN.as_ref());
        audio_process
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .args(audio_args.as_slice());

        println!("Starting audio stream with args: {:?}", audio_args);

        let process_id = Uuid::new_v4().to_hyphenated().to_string();

        let mut audio_process = TranscodeHandler::new(process_id.clone(), audio_process.spawn()?);

        Ok(Self {
            process: stoppable_thread::spawn(move |signal| audio_process.handle(signal)),
            process_id,
            start_number,
            last_chunk: start_number,
            stream_type: StreamType::Audio,
        })
    }

    fn build_video(file: &'a str, start_num: u64, mut profile: Vec<&'a str>) -> Vec<&'a str> {
        let mut args = vec![
            "-ss",
            string_to_static_str((start_num * CHUNK_SIZE).to_string()),
            "-i",
            file,
        ]; // weighted -ss
        args.append(&mut vec!["-copyts", "-map", "0:0"]);
        args.append(&mut profile);
        args.append(&mut vec![
            "-f",
            "hls",
            "-start_number",
            string_to_static_str(start_num.to_string()),
        ]);
        args.append(&mut vec![
            "-hls_time",
            string_to_static_str(CHUNK_SIZE.to_string()),
            "-force_key_frames",
            "expr:gte(t,n_forced*5)",
        ]);
        args.append(&mut vec!["-hls_segment_type", "1"]);
        args.append(&mut vec!["-loglevel", "error", "-progress", "pipe:1"]);
        args
    }

    fn build_audio(file: &'a str, start_num: u64) -> Vec<&'a str> {
        let mut args = vec![
            "-ss",
            string_to_static_str((start_num * CHUNK_SIZE).to_string()),
            "-i",
            file,
        ];
        args.append(&mut vec![
            "-copyts", "-map", "0:1", "-c:0", "aac", "-ac", "2", "-ab", "0",
        ]);
        args.append(&mut vec![
            "-f",
            "hls",
            "-start_number",
            string_to_static_str(start_num.to_string()),
        ]);
        args.append(&mut vec![
            "-hls_time",
            string_to_static_str(CHUNK_SIZE.to_string()),
            "-force_key_frames",
            "expr:gte(t,n_forced*5)",
        ]);
        args.append(&mut vec!["-hls_segment_type", "1"]);
        args.append(&mut vec!["-loglevel", "error", "-progress", "pipe:1"]);
        args
    }

    pub fn join(self) {
        let _ = self.process.stop().join();
    }

    pub fn current_chunk(&self) -> u64 {
        let frame = |k: &str| -> Result<u64, std::option::NoneError> {
            {
                let session = STREAMING_SESSION.read().unwrap();
                Ok(session
                    .get(&self.process_id)?
                    .get(k)?
                    .parse::<u64>()
                    .unwrap_or(0))
            }
        };

        match self.stream_type {
            StreamType::Audio => {
                (frame("out_time_ms").unwrap_or(0) / (CHUNK_SIZE * 1000)
                    + (self.start_number * (CHUNK_SIZE * 1000)))
                    / 5000
            }
            StreamType::Video => {
                frame("frame").unwrap_or(0) / (CHUNK_SIZE * 24) + self.start_number
            }
        }
    }

    /// Method does some math magic to guess if a chunk has been fully written by ffmpeg yet
    pub fn is_chunk_done(&self, chunk_num: u64) -> bool {
        self.current_chunk() > chunk_num
    }

    pub fn is_timeout(&self) -> bool {
        self.current_chunk() >= self.last_chunk + MAX_CHUNKS_AHEAD
    }

    pub fn reset_timeout(&mut self, last_requested: u64) {
        // NOTE: experiment between setting last_chunk to current chunk or taking in the last chunk
        // requested
        if self.current_chunk() < last_requested + MAX_CHUNKS_AHEAD {
            self.last_chunk = self.current_chunk();
        }
    }
}

impl TranscodeHandler {
    fn new(id: String, process: Child) -> Self {
        Self { id, process }
    }

    fn handle(&mut self, signal: &SimpleAtomicBool) {
        let mut stdio = BufReader::new(self.process.stdout.take().unwrap());
        let mut map: HashMap<String, String> = {
            let mut map = HashMap::new();
            map.insert("frame".into(), "0".into());
            map.insert("fps".into(), "0.0".into());
            map.insert("stream_0_0_q".into(), "0.0".into());
            map.insert("bitrate".into(), "0.0kbits/s".into());
            map.insert("total_size".into(), "0".into());
            map.insert("out_time_ms".into(), "0".into());
            map.insert("out_time".into(), "00:00:00.000000".into());
            map.insert("dup_frames".into(), "0".into());
            map.insert("drop_frames".into(), "0".into());
            map.insert("speed".into(), "0.00x".into());
            map.insert("progress".into(), "continue".into());
            map
        };
        let mut out: [u8; 256] = [0; 256];

        'stdout: while !signal.get() {
            let _ = stdio.read_exact(&mut out);
            let output = String::from_utf8_lossy(&out);
            let mut pairs = output
                .lines()
                .map(|x| x.split('=').filter(|x| x.len() > 1).collect::<Vec<&str>>())
                .filter(|x| x.len() == 2)
                .collect::<Vec<Vec<&str>>>();

            pairs.dedup_by(|a, b| a[0].eq(b[0]));

            for pair in pairs {
                if let Some(v) = map.get_mut(&pair[0].to_string()) {
                    *v = pair[1].into();
                }
            }

            {
                let mut lock = STREAMING_SESSION.write().unwrap();
                let _ = lock.insert(self.id.clone(), map.clone());
            }

            match self.process.try_wait() {
                Ok(Some(_)) => break 'stdout,
                Ok(None) => {}
                Err(x) => println!("handle_stdout got err on try_wait(): {:?}", x),
            }
        }

        let _ = self.process.kill();
        let _ = self.process.wait();

        let mut lock = STREAMING_SESSION.write().unwrap();
        let _ = lock.remove(&self.id);
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
