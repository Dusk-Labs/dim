use nightfall::profiles::SideDataList;
use serde::de;
use serde_derive::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::process::Stdio;
use tokio::process::Command;
use tracing::error;
use tracing::trace;

#[derive(Clone, Copy, Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// ffprobe exited early with an error.
    FfprobeError,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FFPStream {
    streams: Vec<Stream>,
    format: Format,
    #[serde(default)]
    corrupt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaConfiguration {
    pub stream_index: i64,
    #[serde(rename = "type")]
    pub media_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoConfiguration {
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub width: i64,
    pub height: i64,
    pub bitrate: i64,
    pub framerate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioConfiguration {
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub channels: i64,
    pub bitrate: i64,
    pub sameplerate: i64,
}

impl Default for FFPStream {
    fn default() -> Self {
        Self {
            corrupt: true,
            streams: Default::default(),
            format: Default::default(),
        }
    }
}

fn deserialize_fraction_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    if let Some((numerator_s, divisor_s)) = s.split_once("/") {
        let numerator: f64 = numerator_s.parse().unwrap();
        let divisor: f64 = divisor_s.parse().unwrap();
        return Ok(Some(numerator / divisor));
    }
    Err(de::Error::custom("Expected a string such as '24000/1001'"))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stream {
    pub index: i64,
    pub codec_name: String,
    pub profile: Option<String>,
    pub codec_type: String,
    pub codec_time_base: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub coded_width: Option<i64>,
    pub coded_height: Option<i64>,
    pub display_aspect_ratio: Option<String>,
    pub is_avc: Option<String>,
    pub has_b_frames: Option<u64>,
    pub pix_fmt: Option<String>,
    #[serde(deserialize_with = "deserialize_fraction_string")]
    pub r_frame_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_fraction_string")]
    pub avg_frame_rate: Option<f64>,
    pub level: Option<i64>,
    pub tags: Option<Tags>,
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    pub channel_layout: Option<String>,
    pub bit_rate: Option<String>,
    pub duration_ts: Option<i64>,
    pub duration: Option<String>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub disposition: Option<Disposition>,
    pub side_data_list: Option<Vec<SideDataList>>,
}

impl Stream {
    pub fn get_bitrate(&self) -> Option<u64> {
        self.tags.as_ref()?.bps_eng.as_ref()?.parse::<u64>().ok()
    }

    pub fn get_codec(&self) -> String {
        match self.codec_name.as_str() {
            "h264" => format!("avc1.6400{:x}", self.level.unwrap_or(40)),
            "av1" => "av01.0.01M.08".into(),
            "aac" => "mp4a.40.2".into(),
            "ac3" => "ac-3".into(),
            "eac3" => "ec-3".into(),
            "dts" => "dtsh".into(),
            "truehd" => "mlpa".into(),
            "vp8" => "vp8".into(),
            "vp9" => "vp09.00.10.08".into(),
            "hevc" => {
                if let Some(side_data_list) = &self.side_data_list {
                    format!(
                        "dvh1.{:02}.{:02}",
                        side_data_list[0].dv_profile.unwrap_or(5),
                        side_data_list[0].dv_level.unwrap_or(6)
                    )
                } else {
                    "hvc1.2.4.L153.B0".into()
                }
            }
            _ => "Unknown".into(),
        }
    }

    pub fn get_language(&self) -> Option<String> {
        self.tags.as_ref()?.language.clone()
    }

    pub fn get_title(&self) -> Option<String> {
        self.tags.as_ref()?.title.clone()
    }
}

impl From<Stream> for nightfall::profiles::InputCtx {
    fn from(stream: Stream) -> nightfall::profiles::InputCtx {
        nightfall::profiles::InputCtx {
            stream: stream.index as usize,
            codec: stream.codec_name,
            pix_fmt: stream.pix_fmt.unwrap_or_default(),
            profile: stream.profile.unwrap_or_default(),
            bitrate: stream
                .tags
                .and_then(|x| x.bps_eng?.parse::<u64>().ok())
                .unwrap_or_default(),
            bframes: stream.has_b_frames,
            audio_channels: stream.channels.unwrap_or(2) as u64,
            side_data_list: stream.side_data_list,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    pub language: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "BPS-eng")]
    pub bps_eng: Option<String>,
    #[serde(rename = "DURATION-eng")]
    duration_eng: Option<String>,
    #[serde(rename = "NUMBER_OF_FRAMES-eng")]
    number_of_frames_eng: Option<String>,
    #[serde(rename = "_STATISTICS_WRITING_APP-eng")]
    statistics_writing_app_eng: Option<String>,
    #[serde(rename = "_STATISTICS_WRITING_DATE_UTC-eng")]
    statistics_writing_date_utc_eng: Option<String>,
    #[serde(rename = "_STATISTICS_TAGS-eng")]
    statistics_tags_eng: Option<String>,
    filename: Option<String>,
    mimetype: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Format {
    pub filename: String,
    pub nb_streams: i64,
    pub nb_programs: i64,
    pub format_name: String,
    pub format_long_name: String,
    pub start_time: String,
    pub duration: String,
    pub size: String,
    pub bit_rate: String,
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

    #[tracing::instrument(skip(self, file))]
    pub async fn get_meta(&self, file: impl ToString) -> Result<FFPStream, std::io::Error> {
        let mut probe = Command::new(self.ffprobe_bin.clone());

        probe
            .arg(file.to_string())
            .arg("-v")
            .arg("error")
            .arg("-print_format")
            .arg("json")
            .arg("-show_streams")
            .arg("-show_format")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        trace!(
            binary = self.ffprobe_bin.as_str(),
            args = %probe.as_std().get_args().filter_map(|x| x.to_str()).collect::<Vec<_>>().join(" "),
            "Spawning ffprobe."
        );

        let output = probe.spawn()?.wait_with_output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            error!(status = ?output.status, stderr = %stderr, "ffprobe exited with an error status.");
        }

        let json = String::from_utf8_lossy(output.stdout.as_slice());

        let de = serde_json::from_str(&json).unwrap_or_default();

        Ok(de)
    }
}

impl FFPStream {
    pub fn get_container(&self) -> String {
        self.format.format_name.clone()
    }

    pub fn get_primary_channels(&self) -> Option<i64> {
        self.get_primary("audio")?.channels
    }

    pub fn get_audio_lang(&self) -> Option<String> {
        self.get_primary("audio")?.get_language()
    }

    pub fn get_video_lang(&self) -> Option<String> {
        self.get_primary("video")?.get_language()
    }

    pub fn get_container_bitrate(&self) -> Option<u64> {
        self.format.bit_rate.parse::<u64>().ok()
    }

    pub fn get_video_codec(&self) -> Option<String> {
        Some(self.find_by_type("video").first()?.codec_name.clone())
    }

    pub fn get_video_profile(&self) -> Option<String> {
        self.find_by_type("video").first()?.profile.clone()
    }

    pub fn get_height(&self) -> Option<i64> {
        self.find_by_type("video").first()?.height
    }

    pub fn get_width(&self) -> Option<i64> {
        self.find_by_type("video").first()?.width
    }

    pub fn get_primary(&self, codec_type: &str) -> Option<&Stream> {
        let mut streams: VecDeque<_> = self.find_by_type(codec_type).into();

        if streams.is_empty() {
            return None;
        }

        if streams.len() == 1 {
            return streams.pop_front();
        }

        let primary_stream = streams.iter().find_map(|x| {
            if x.disposition.as_ref()?.default == 1 {
                Some(*x)
            } else {
                None
            }
        });

        primary_stream.or_else(|| streams.pop_front())
    }

    pub fn get_primary_codec(&self, codec_type: &str) -> Option<&str> {
        Some(&self.get_primary(codec_type)?.codec_name)
    }

    pub fn get_duration(&self) -> Option<i32> {
        Some(self.format.duration.parse::<f64>().ok()? as i32)
    }

    pub fn get_ms(&self) -> Option<u128> {
        self.format
            .duration
            .parse::<f64>()
            .map(|x| (x.trunc() * 1_000_000.0) as u128)
            .ok()
    }

    pub fn is_corrupt(&self) -> bool {
        self.corrupt
    }

    pub fn is_codec_type(&self, codec_type: &str) -> Option<bool> {
        Some(!self.find_by_type(codec_type).is_empty())
    }

    pub fn find_by_type(&self, codec_type: &str) -> Vec<&Stream> {
        self.streams
            .iter()
            .filter(|x| x.codec_type == *codec_type)
            .collect()
    }

    pub fn get_media_configurations(&self) -> Vec<MediaConfiguration> {
        self.streams
            .iter()
            .filter(|stream| stream.codec_type == "audio" || stream.codec_type == "video")
            .map(|stream| {
                let mut m = MediaConfiguration {
                    stream_index: stream.index,
                    media_type: "media-source".into(),
                    video: None,
                    audio: None,
                };
                if stream.codec_type == "video" {
                    m.video = Some(VideoConfiguration {
                        content_type: format!("video/mp4;codecs={}", stream.get_codec()).into(),
                        width: stream.width.unwrap_or(1920),
                        height: stream.height.unwrap_or(1080),
                        bitrate: stream
                            .clone()
                            .bit_rate
                            .unwrap_or("1000".into())
                            .parse::<i64>()
                            .unwrap_or(1000),
                        framerate: stream.avg_frame_rate.unwrap_or(24 as f64),
                    })
                } else {
                    m.audio = Some(AudioConfiguration {
                        content_type: format!("audio/mp4;codecs={}", stream.get_codec()).into(),
                        channels: stream.channels.unwrap_or(2),
                        bitrate: stream
                            .clone()
                            .bit_rate
                            .unwrap_or("1000".into())
                            .parse::<i64>()
                            .unwrap_or(1000),
                        sameplerate: stream
                            .clone()
                            .sample_rate
                            .unwrap_or("24000".into())
                            .parse::<i64>()
                            .unwrap_or(24000),
                    })
                }
                m
            })
            .collect::<Vec<MediaConfiguration>>()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Serialize)]
pub struct Disposition {
    pub default: i64,
    pub dub: i64,
    pub original: i64,
    pub comment: i64,
    pub lyrics: i64,
    pub karaoke: i64,
    pub forced: i64,
    pub hearing_impaired: i64,
    pub visual_impaired: i64,
}
