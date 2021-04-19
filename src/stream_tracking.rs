use crate::core::StateManager;
use auth::Wrapper as Auth;
use rocket::State;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use tokio::runtime::Handle;

pub struct StreamTracking {
    streaming_sessions: Arc<RwLock<HashMap<u128, Vec<String>>>>,
}

impl StreamTracking {
    pub fn insert(&self, id: u128, stream_id: String) {
        let mut lock = self.streaming_sessions.write().unwrap();
        lock.entry(id).or_default().push(stream_id);
    }

    pub fn kill_all(&self, tokio_rt: &State<Handle>, state: &State<StateManager>, id: u128) {
        let mut lock = self.streaming_sessions.write().unwrap();

        if let Some(v) = lock.get_mut(&id) {
            if !v.is_empty() {
                for id in v.drain(..) {
                    let _ = tokio_rt.block_on(state.die(id));
                }
            }
        }
    }

    pub fn get_for_gid(&self, gid: u128) -> Option<Vec<String>> {
        let lock = self.streaming_sessions.read().unwrap();
        lock.get(&gid).cloned()
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
