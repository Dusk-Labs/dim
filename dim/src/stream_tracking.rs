use std::collections::HashMap;
use std::sync::Arc;
use std::num::NonZeroU64;

use crate::core::StateManager;
use crate::utils::ts_to_xml;
use tokio::sync::RwLock;
use uuid::Uuid;

use serde::Serialize;
use xmlwriter::*;

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
    pub set_id: NonZeroU64,
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
    pub label: String,
    pub lang: Option<String>,
}

impl VirtualManifest {
    pub fn compile(&self, w: &mut XmlWriter, start_num: u64) {
        match self.content_type {
            ContentType::Subtitle => self.compile_sub(w),
            _ => self.compile_av(w, start_num),
        }
    }

    fn compile_av(&self, w: &mut XmlWriter, start_num: u64) {
        if matches!(self.content_type, ContentType::Audio) {
            // Each audio stream must be in a separate adaptation set otherwise theyre treated as
            // different bitrates of the same track rather than separate tracks.
            w.start_element("AdaptationSet");
            w.write_attribute("contentType", "audio");
            w.write_attribute("id", &self.set_id);
            
            if let Some(lang) = self.lang.as_ref() {
                w.write_attribute("lang", lang);
            }
        }

        let init = format!("{}?start_num={}", self.init_seg.clone().unwrap(), start_num);
        let chunk_path = self.chunk_path.clone();

        // write representations
        w.start_element("Representation");
        w.write_attribute("id", &self.id);
        w.write_attribute("bandwidth", &self.bandwidth);
        w.write_attribute("mimeType", &self.mime);
        w.write_attribute("codecs", &self.codecs);
        
        for (k, v) in self.args.iter() {
            w.write_attribute(k, v);
        }

        // write audio channel config
        if matches!(self.content_type, ContentType::Audio) {
            w.start_element("AudioChannelConfiguration");
            w.write_attribute("schemeIdUri", "urn:mpeg:dash:23003:3:audio_channel_configuration:2011");
            w.write_attribute("value", "2"); // FIXME: At some point we need to stop hardcoding 2ch audio
            w.end_element();
        }

        // write segment template
        w.start_element("SegmentTemplate");
        w.write_attribute("timescale", &1000);
        w.write_attribute("duration", &5000);
        w.write_attribute("initialization", &init);
        w.write_attribute("media", &chunk_path);
        w.write_attribute("startNumber", &start_num);

        // close SegmentTemplate and Representation
        w.end_element();
        w.end_element();

        if matches!(self.content_type, ContentType::Audio) {
            w.end_element(); // close AdapationSet
        }
    }

    fn compile_sub(&self, w: &mut XmlWriter) {
        w.start_element("AdapationSet");
        w.write_attribute("mimeType", &self.mime);
        w.write_attribute("id", &self.set_id);

        if let Some(lang) = self.lang.as_ref() {
            w.write_attribute("lang", lang);
        }
        
        for (k, v) in self.args.iter() {
            w.write_attribute(k, v);
        }

        w.start_element("Representation");
        w.write_attribute("id", &self.id);
        w.write_attribute("bandwidth", &self.bandwidth);
        
        w.start_element("BaseURL");
        w.write_text(&self.chunk_path);
        w.end_element();
        w.end_element();
        w.end_element();
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
        let duration = ts_to_xml(manifests.first().and_then(|x| x.duration)? as u64);

        let mut w = XmlWriter::new(Default::default());
        w.write_declaration();

        w.start_element("MPD");
        w.write_attribute("xmlns", "urn:mpeg:dash:schema:mpd:2011");
        w.write_attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
        w.write_attribute("xsi:schemaLocation", "urn:mpeg:dash:schema:mpd:2011 http://standards.iso.org/ittf/PubliclyAvailableStandards/MPEG-DASH_schema_files/DASH-MPD.xsd");
        w.write_attribute("profiles", "urn:mpeg:dash:profile:full:2011");
        w.write_attribute("type", "static");
        w.write_attribute("mediaPresentationDuration", &duration);
        w.write_attribute("minBufferTime", "PT20S");
        w.write_attribute("maxSegmentDuration", "PT20S");

        w.start_element("Period");
        w.write_attribute("duration", &duration);
        w.start_element("BaseURL");
        w.write_text("/api/v1/stream/");
        w.end_element();

        // write video tracks within the first adaptation set.
        w.start_element("AdaptationSet");
        w.write_attribute("contentType", "video");
        w.write_attribute("id", "0");

        for track in manifests {
            if matches!(track.content_type, ContentType::Video) {
                track.compile(&mut w, start_num);
            }
        }
        w.end_element();

        // write the audio and subtitle tracks.
        for track in manifests {
            if matches!(track.content_type, ContentType::Audio | ContentType::Subtitle) {
                track.compile(&mut w, start_num);
            }
        }

        Some(w.end_document())
    }

    pub async fn compile_only(
        &self,
        gid: &Uuid,
        start_num: u64,
        _filter: Vec<String>,
    ) -> Option<String> {
        self.compile(gid, start_num).await
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
