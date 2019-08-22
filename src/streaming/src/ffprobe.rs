use std::path::PathBuf;
use std::process::Command;
use std::str;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FFPStream {
    streams: Vec<Stream>,
    format: Format,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Stream {
    index: i64,
    codec_name: String,
    codec_long_name: String,
    profile: Option<String>,
    codec_type: String,
    codec_time_base: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    coded_width: Option<i64>,
    coded_height: Option<i64>,
    display_aspect_ratio: Option<String>,
    is_avc: Option<String>,
    tags: Option<Tags>,
    sample_rate: Option<String>,
    channels: Option<i64>,
    channel_layout: Option<String>,
    bit_rate: Option<String>,
    duration_ts: Option<i64>,
    duration: Option<String>,
    color_range: Option<String>,
    color_space: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Tags {
    language: Option<String>,
    title: Option<String>,
    #[serde(rename = "BPS-eng")]
    bps_eng: Option<String>,
    #[serde(rename = "DURATION-eng")]
    duration_eng: Option<String>,
    #[serde(rename = "NUMBER_OF_FRAMES-eng")]
    number_of_frames_eng: Option<String>,
    #[serde(rename = "NUMBER_OF_BYTES-eng")]
    number_of_bytes_eng: Option<String>,
    #[serde(rename = "_STATISTICS_WRITING_APP-eng")]
    statistics_writing_app_eng: Option<String>,
    #[serde(rename = "_STATISTICS_WRITING_DATE_UTC-eng")]
    statistics_writing_date_utc_eng: Option<String>,
    #[serde(rename = "_STATISTICS_TAGS-eng")]
    statistics_tags_eng: Option<String>,
    filename: Option<String>,
    mimetype: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Format {
    filename: String,
    nb_streams: i64,
    nb_programs: i64,
    format_name: String,
    format_long_name: String,
    start_time: String,
    duration: String,
    size: String,
    bit_rate: String,
}

pub struct FFProbeCtx {
    ffprobe_bin: String,
}

impl FFProbeCtx {
    pub fn new(ffprobe_bin: &'static str) -> Self {
        Self {
            ffprobe_bin: ffprobe_bin.to_owned(),
        }
    }

    pub fn get_meta(&self, file: &PathBuf) -> Result<FFPStream, std::io::Error> {
        let probe = Command::new(self.ffprobe_bin.clone())
            .arg(file.to_str().unwrap())
            .arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_streams")
            .arg("-show_format")
            .output()?;

        let json = String::from_utf8_lossy(probe.stdout.as_slice());

        let de: FFPStream = serde_json::from_str(&json).unwrap();

        Ok(de)
    }
}

impl FFPStream {
    pub fn get_quality(&self) -> Option<String> {
        match self.streams[0].height {
            Some(x) => Some(x.to_string()),
            None => None,
        }
    }

    pub fn get_codec(&self) -> Option<String> {
        Some(self.streams[0].codec_name.clone())
    }

    pub fn get_container(&self) -> Option<String> {
        Some(self.format.format_name.clone())
    }

    pub fn get_audio_type(&self) -> Option<String> {
        Some(self.streams[1].codec_long_name.clone())
    }

    pub fn get_res(&self) -> Option<String> {
        Some(format!(
            "{}x{}",
            self.streams[0].width.unwrap_or(1920),
            self.streams[0].height.unwrap_or(1080)
        ))
    }

    pub fn get_duration(&self) -> Option<i32> {
        Some(self.format.duration.parse::<f64>().unwrap() as i32)
    }
}
