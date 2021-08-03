use std::collections::HashMap;
use std::sync::Arc;

use crate::core::StateManager;
use tokio::sync::RwLock;
use uuid::Uuid;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Video,
    Audio,
    Subtitle,
}

#[derive(Debug, Clone, Serialize)]
pub struct VirtualManifest {
    pub content_type: ContentType,
    pub id: String,
    pub is_direct: bool,
    pub mime: String,
    pub codecs: String,
    pub bandwidth: u64,
    #[serde(flatten)]
    pub args: HashMap<String, String>,
    pub duration: Option<i32>,
    pub chunk_path: String,
    pub init_seg: Option<String>,
    pub is_default: bool,
}

impl VirtualManifest {
    pub fn compile(&self, start_num: u64) -> Option<String> {
        match self.content_type {
            ContentType::Video => self.compile_video(start_num),
            ContentType::Audio => self.compile_audio(start_num),
            ContentType::Subtitle => self.compile_sub(),
        }
    }

    fn compile_video(&self, start_num: u64) -> Option<String> {
        Some(format!(
            include_str!("./static/video_segment.mpd"),
            id = &self.id,
            bandwidth = self.bandwidth.to_string(),
            mimeType = &self.mime,
            avc = &self.codecs,
            init = format!("{}?start_num={}", self.init_seg.clone().unwrap(), start_num),
            chunk_path = self.chunk_path.clone(),
            start_num = start_num,
            args = self
                .args
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<_>>()
                .join(" ")
        ))
    }

    fn compile_audio(&self, start_num: u64) -> Option<String> {
        Some(format!(
            include_str!("./static/audio_segment.mpd"),
            id = &self.id,
            bandwidth = self.bandwidth.to_string(),
            mimeType = &self.mime,
            codecs = &self.codecs,
            init = format!("{}?start_num={}", self.init_seg.clone().unwrap(), start_num),
            chunk_path = self.chunk_path.clone(),
            start_num = start_num,
        ))
    }

    fn compile_sub(&self) -> Option<String> {
        Some(format!(
            include_str!("./static/subtitle_segment.mpd"),
            id = &self.id,
            bandwidth = self.bandwidth.to_string(),
            mimeType = &self.mime,
            path = self.chunk_path.clone(),
            args = self
                .args
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<_>>()
                .join(" ")
        ))
    }
}

pub struct StreamTracking {
    streaming_sessions: Arc<RwLock<HashMap<Uuid, Vec<VirtualManifest>>>>,
}

impl StreamTracking {
    pub async fn insert(&self, id: &Uuid, manifest: VirtualManifest) {
        let mut lock = self.streaming_sessions.write().await;
        lock.entry(*id).or_default().push(manifest);
    }

    pub async fn kill_all(&self, state: &StateManager, id: &Uuid, ignore_gc: bool) {
        let mut lock = self.streaming_sessions.write().await;

        if let Some(v) = lock.get_mut(id) {
            if !v.is_empty() {
                for manifest in v {
                    let _ = if ignore_gc {
                        state.die_ignore_gc(manifest.id.clone()).await
                    } else {
                        state.die(manifest.id.clone()).await
                    };
                }
            }
        }
    }

    pub async fn kill(&self, state: &StateManager, gid: &Uuid, ids: Vec<String>, ignore_gc: bool) {
        let lock = self.streaming_sessions.read().await;

        if let Some(v) = lock.get(gid) {
            if !v.is_empty() {
                for id in ids {
                    let _ = if ignore_gc {
                        state.die_ignore_gc(id).await
                    } else {
                        state.die(id).await
                    };
                }
            }
        }
    }

    pub async fn get_for_gid(&self, gid: &Uuid) -> Vec<VirtualManifest> {
        let lock = self.streaming_sessions.read().await;
        lock.get(gid).cloned().unwrap_or_default()
    }

    pub async fn compile(&self, gid: &Uuid, start_num: u64) -> Option<String> {
        let lock = self.streaming_sessions.read().await;
        let manifests = lock.get(gid)?;

        let tracks = manifests
            .iter()
            .filter_map(|x| x.compile(start_num))
            .collect::<Vec<_>>();

        let duration = manifests.first().and_then(|x| x.duration)?;

        Some(format!(
            include_str!("./static/manifest.mpd"),
            duration = format!("PT{}S", duration),
            base_url = "/api/v1/stream/",
            segments = tracks.join("\n")
        ))
    }

    pub async fn compile_only(
        &self,
        gid: &Uuid,
        start_num: u64,
        filter: Vec<String>,
    ) -> Option<String> {
        let lock = self.streaming_sessions.read().await;
        let manifests = lock.get(gid)?;

        let tracks = manifests
            .iter()
            .filter(|x| filter.contains(&x.id))
            .filter_map(|x| x.compile(start_num))
            .collect::<Vec<_>>();

        let duration = manifests.first().and_then(|x| x.duration)?;

        Some(format!(
            include_str!("./static/manifest.mpd"),
            duration = duration,
            base_url = "/api/v1/stream/",
            segments = tracks.join("\n")
        ))
    }
}

impl Default for StreamTracking {
    fn default() -> Self {
        Self {
            streaming_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Clone for StreamTracking {
    fn clone(&self) -> Self {
        Self {
            streaming_sessions: Arc::clone(&self.streaming_sessions),
        }
    }
}
