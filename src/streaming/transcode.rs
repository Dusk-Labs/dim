use crate::errors;
use std::process::{Child, Command, Stdio};

const CHUNK_SIZE: u64 = 5;

pub struct Session {
    audio_process: Child,
    video_process: Child,
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

        let mut video_process = Command::new("/usr/local/bin/ffmpeg");
        video_process
            .stdout(Stdio::piped())
            .args(video_args.as_slice());

        let mut audio_process = Command::new("/usr/local/bin/ffmpeg");
        audio_process
            .stdout(Stdio::piped())
            .args(audio_args.as_slice());

        println!("{:?}", video_args);
        println!("{:?}", audio_args);

        let video_process = video_process.spawn()?;
        let audio_process = audio_process.spawn()?;

        Ok(Self {
            video_process,
            audio_process,
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

    pub fn join(mut self) {
        let _ = self.audio_process.kill();
        let _ = self.audio_process.wait();

        let _ = self.video_process.kill();
        let _ = self.video_process.wait();
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
