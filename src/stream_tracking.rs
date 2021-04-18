use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;

use rocket::http::ContentType;
use rocket::http::Method;
use rocket::http::Status;
use rocket::request::FromRequest;

use rocket::Data;
use rocket::Outcome;
use rocket::Request;
use rocket::Response;
use rocket::State;

use crate::core::StateManager;
use nightfall::Die;

use auth::Wrapper as Auth;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

const PATHS_TO_FILTER: &'static [&'static str] = &["/api/v1/stream", "api/v1/stream"];

pub struct StreamTracking {
    streaming_sessions: Arc<RwLock<HashMap<u128, Vec<String>>>>,
}

impl StreamTracking {
    pub fn insert(&self, id: u128, stream_id: String) {
        let mut lock = self.streaming_sessions.write().unwrap();
        lock.entry(id).or_default().push(stream_id);
    }

    pub fn kill_all(&self, state: &State<StateManager>, id: u128) {
        let mut lock = self.streaming_sessions.write().unwrap();

        if let Some(v) = lock.get_mut(&id) {
            if !v.is_empty() {
                for id in v.drain(..) {
                    let _ = state.do_send(Die(id));
                }
            }
        }
    }

    pub fn get_for_gid(&self, gid: u128) -> Option<Vec<String>> {
        let lock = self.streaming_sessions.read().unwrap();
        lock.get(&gid).cloned()
    }
}

impl Fairing for StreamTracking {
    fn info(&self) -> Info {
        Info {
            name: "StreamTracker",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, data: &Data) {
        let token = match request.guard::<Auth>() {
            Outcome::Success(x) => x,
            _ => return,
        };

        let route = request
            .uri()
            .segments()
            .take(3)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("/");

        if PATHS_TO_FILTER.contains(&route.as_str()) {
            return;
        }

        if let Outcome::Success(sessions) = request.guard::<State<StateManager>>() {
            self.kill_all(&sessions, token.0.claims.id);
        }
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
