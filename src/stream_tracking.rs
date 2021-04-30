use crate::core::StateManager;
use auth::Wrapper as Auth;
use rocket::State;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::runtime::Handle;
use tokio::sync::RwLock;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ContentType {
    Video,
    Audio,
    Subtitle,
}

#[derive(Debug, Clone, Serialize)]
pub struct VirtualManifest {
    pub content_type: ContentType,
    pub id: String,
    pub mime: String,
    pub codecs: String,
    pub bandwidth: u64,
    #[serde(flatten)]
    pub args: HashMap<String, String>,
    pub duration: Option<i32>,
    pub chunk_path: String,
    pub init_seg: Option<String>,
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
        todo!()
    }

    fn compile_sub(&self) -> Option<String> {
        todo!()
    }
}

pub struct StreamTracking {
    streaming_sessions: Arc<RwLock<HashMap<u128, Vec<VirtualManifest>>>>,
}

impl StreamTracking {
    pub async fn insert(&self, id: u128, manifest: VirtualManifest) {
        let mut lock = self.streaming_sessions.write().await;
        lock.entry(id).or_default().push(manifest);
    }

    pub async fn kill_all(&self, state: &State<'_, StateManager>, id: u128) {
        let mut lock = self.streaming_sessions.write().await;

        if let Some(v) = lock.get_mut(&id) {
            if !v.is_empty() {
                for manifest in v.drain(..) {
                    let _ = state.die(manifest.id).await;
                }
            }
        }
    }

    pub async fn get_for_gid(&self, gid: u128) -> Vec<VirtualManifest> {
        let lock = self.streaming_sessions.read().await;
        lock.get(&gid).cloned().unwrap_or_default()
    }

    pub async fn compile(&self, gid: u128, start_num: u64) -> Option<String> {
        let lock = self.streaming_sessions.read().await;
        let manifests = lock.get(&gid)?;

        let tracks = manifests
            .iter()
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

    pub async fn compile_only(
        &self,
        gid: u128,
        start_num: u64,
        filter: Vec<String>,
    ) -> Option<String> {
        let lock = self.streaming_sessions.read().await;
        let manifests = lock.get(&gid)?;

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
