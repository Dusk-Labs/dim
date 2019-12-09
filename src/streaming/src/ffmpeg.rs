use database::{get_conn_logged, mediafile::MediaFile};
use events::{Message, PushEventType};
use pushevent::Event;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use stoppable_thread::{self, SimpleAtomicBool, StoppableHandle};
use uuid::Uuid;

pub type EventTx = std::sync::mpsc::Sender<pushevent::Event>;

lazy_static::lazy_static! {
    static ref STREAMS: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref STDIO_HANDLES: Arc<Mutex<HashMap<String, StoppableHandle<()>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct FFmpeg {
    bin: String,
    mediafile: MediaFile,
    uuid: String,
    out_dir: String,
    event_tx: EventTx,
}

impl FFmpeg {
    pub fn new(
        ffmpeg_bin: &'static str,
        mediafile: i32,
        event_tx: EventTx,
        log: slog::Logger,
    ) -> Result<Self, ()> {
        let uuid = Uuid::new_v4();
        let conn = match get_conn_logged(&log) {
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
            out_dir: format!("transcoding/{}", uuid.to_hyphenated().to_string()),
            event_tx,
        })
    }

    /// TODO: Add params to select codec out and in, seek, and further params.
    pub fn stream(&mut self, seek: u64) -> Result<String, ()> {
        let input = format!("file:{}", self.mediafile.target_file.clone().as_str());
        let manifest = format!("{}/index.m3u8", self.out_dir);
        let chunks = format!("{}/%d.ts", self.out_dir);
        /*
        let progress_url = format!("http://127.0.0.1:8000/api/v1/stream/{}/progress", self.uuid);
        let progress_url = format!("./logs/ffmpeg-{}.log", self.uuid);
        */
        let progress_url = "pipe:1".to_string();

        let time_seek = format!("{}", seek);

        let _ = fs::create_dir(self.out_dir.clone());

        let mut process = Command::new(self.bin.clone());
        process
            .stdout(Stdio::piped())
            .args(&[
                "-ss",
                time_seek.as_str(),
                "-fflags",
                "+genpts",
                "-noaccurate_seek",
            ])
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
            .args(&["-segment_list", manifest.as_str(), "-y", chunks.as_str()])
            .args(&["-loglevel", "error"])
            .args(&["-progress", progress_url.as_str()]);

        let process = match process.spawn() {
            Ok(process) => process,
            Err(_) => return Err(()),
        };

        {
            let tx = self.event_tx.clone();
            let uuid = self.uuid.clone();
            STDIO_HANDLES.lock().unwrap().insert(
                self.uuid.clone(),
                stoppable_thread::spawn(move |signal| {
                    FFmpeg::handle_stdout(uuid, signal, process, tx)
                }),
            );
        }

        Ok(self.uuid.clone())
    }

    // NOTE: This will not work on windows and only on nix
    // TODO: Figure out how to get rid of zombie processes
    pub fn stop(uuid: String) -> Result<(), std::io::Error> {
        if let Some(handle) = STDIO_HANDLES.lock().unwrap().remove(&uuid) {
            handle.stop().join().unwrap();
        }

        Ok(())
    }

    pub fn handle_stdout(
        stream_uuid: String,
        signal: &SimpleAtomicBool,
        mut proc: Child,
        event_tx: EventTx,
    ) {
        use std::io::BufReader;
        let mut stdio = BufReader::new(proc.stdout.take().unwrap());
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

        while !signal.get() {
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

            let new_event = Box::new(Message {
                id: -1, // we dont need a id for this so we ignore it with -1
                event_type: PushEventType::EventStreamStats(map.clone()),
            });

            let _ = event_tx.send(Event::new(
                format!("/events/stream/{}", stream_uuid),
                new_event,
            ));
        }

        let _ = proc.kill();
        let _ = proc.wait();
    }
}
