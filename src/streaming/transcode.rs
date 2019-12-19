use crate::errors;
use std::collections::HashMap;
use std::io::Read;
use std::process::{Child, Command, Stdio};
use stoppable_thread::{self, SimpleAtomicBool, StoppableHandle};

const CHUNK_SIZE: u64 = 5;

pub struct Session {
    pub video_process_id: String,
    pub audio_process_id: String,
    audio_process: StoppableHandle<()>,
    video_process: StoppableHandle<()>,
}

struct TranscodeHandler {
    id: String,
    process: Child,
}

pub enum Profile {
    High,
    Medium,
    Low,
}

impl Profile {
    fn to_params(self) -> (&'static str, &'static str) {
        match self {
            Self::High => ("4M", "scale=1080:-n"),
            Self::Medium => ("2M", "scale=720:-n"),
            Self::Low => ("1M", "scale=480:-m"),
        }
    }
}

impl<'a> Session {
    /// Function returns a new transcoding session based on the params passed, it also
    /// automatically calculates the hls_start attribute for streams started at an offset
    pub fn new(
        file: String,
        profile: Option<Profile>,
        start_number: u64,
        outdir: String,
    ) -> Result<Self, errors::StreamingErrors> {
        let file = format!("file://{}", file);
        let profile_args = profile.map_or_else(
            || vec!["-c:0", "copy"],
            |x| {
                let x = x.to_params();
                vec!["-c:0", "libx264", "-b:v", x.0, "-preset", "veryfast"]
            },
        );

        let _ = std::fs::create_dir_all(format!("{}/video", outdir.clone()));
        let _ = std::fs::create_dir(format!("{}/audio", outdir.clone()));

        let mut video_args = Self::build_video(
            string_to_static_str(file.clone()),
            start_number,
            profile_args,
        );
        let mut audio_args = Self::build_audio(string_to_static_str(file.clone()), start_number);

        video_args.push("-hls_segment_filename");
        audio_args.push("-hls_segment_filename");
        video_args.push(string_to_static_str(format!("{}/video/%d.m4s", outdir)));
        audio_args.push(string_to_static_str(format!("{}/audio/%d.m4s", outdir)));
        video_args.push(string_to_static_str(format!(
            "{}/video/playlist.m3u8",
            outdir
        )));
        audio_args.push(string_to_static_str(format!(
            "{}/audio/playlist.m3u8",
            outdir
        )));

        let mut video_process = Command::new(super::FFMPEG_BIN.as_ref());
        video_process
            .stdout(Stdio::piped())
            .args(video_args.as_slice());

        let mut audio_process = Command::new(super::FFMPEG_BIN.as_ref());
        audio_process
            .stdout(Stdio::piped())
            .args(audio_args.as_slice());

        println!("{:?}", video_args);
        println!("{:?}", audio_args);

        let video_process_id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let audio_process_id = uuid::Uuid::new_v4().to_hyphenated().to_string();

        let mut video_process =
            TranscodeHandler::new(video_process_id.clone(), video_process.spawn()?);
        let mut audio_process =
            TranscodeHandler::new(audio_process_id.clone(), audio_process.spawn()?);

        Ok(Self {
            video_process: stoppable_thread::spawn(move |signal| video_process.handle(signal)),
            audio_process: stoppable_thread::spawn(move |signal| audio_process.handle(signal)),
            video_process_id,
            audio_process_id,
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
        let _ = self.audio_process.stop().join().unwrap();
        let _ = self.video_process.stop().join().unwrap();
    }
}

impl TranscodeHandler {
    fn new(id: String, process: Child) -> Self {
        Self { id, process }
    }

    fn handle(&mut self, signal: &SimpleAtomicBool) {
        use crate::streaming::STREAMING_SESSION;
        use std::io::BufReader;
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
                let mut lock = STREAMING_SESSION.lock().unwrap();
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

        let mut lock = STREAMING_SESSION.lock().unwrap();
        let _ = lock.remove(&self.id);
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
