use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

use rocket::http::hyper::Server;
use serde::{Deserialize, Serialize};
use tokio::process::Child;
use tokio_tungstenite::tungstenite::Message;
use xtra::{Actor, Handler};

#[derive(Debug)]
pub struct Plugin {
    proc: Child,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ServerState {
    Dead,
    Booting,
    Running,
    ShuttingDown,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::Dead
    }
}

#[derive(Debug, Default)]
pub struct PluginServer {
    state: ServerState,
    subprocs: HashMap<String, Plugin>,
}

impl Actor for PluginServer {}

// -- struct PluginMessage

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginMessage {
    #[serde(rename = "t")]
    pub(super) trigger: String,

    #[serde(rename = "d")]
    pub(super) data: serde_json::Value,

    #[serde(rename = "r")]
    pub(super) reference: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Envelope(pub SocketAddr, pub PluginMessage);

impl From<Envelope> for Message {
    fn from(Envelope(_, ref message): Envelope) -> Self {
        Message::Text(serde_json::to_string(message).unwrap())
    }
}

impl xtra::Message for Envelope {
    type Result = Result<(), PluginMessage>;
}

#[async_trait::async_trait]
impl Handler<Envelope> for PluginServer {
    async fn handle(
        &mut self,
        Envelope(sender, message): Envelope,
        ctx: &mut xtra::Context<Self>,
    ) -> <Envelope as xtra::Message>::Result {
        // probably just gonna be a big event switch from here on out.
        match &message.trigger {
            // TODO: Add some commands.

            unknown => Err(PluginMessage {
                trigger: "UNKNOWN_COMMAND".to_string(),
                reference: message.reference.clone(),
                data: serde_json::to_value(message).unwrap(),
            }),
        }
    }
}

// -- struct Auth(String, Child);

#[derive(Debug)]
pub(super) enum RunnerQuery {
    AddPending { token: String, child: Child },

    NotifyStartupComplete,

    GetServerState { tx: tokio::sync::oneshot::Sender<ServerState> },
}

impl xtra::Message for RunnerQuery {
    type Result = ();
}

#[async_trait::async_trait]
impl Handler<RunnerQuery> for PluginServer {
    async fn handle(&mut self, query: RunnerQuery, ctx: &mut xtra::Context<Self>) {
        match query {
            RunnerQuery::AddPending { token, child }
                if matches!(self.state, ServerState::Dead | ServerState::Booting) =>
            {
                self.subprocs.insert(token, Plugin { proc: child });
                self.state = ServerState::Booting;
            }

            RunnerQuery::NotifyStartupComplete
                if matches!(self.state, ServerState::Dead | ServerState::Booting) =>
            {
                self.state = ServerState::Running;
            }

            RunnerQuery::GetServerState { tx } => { let _ = tx.send(self.state); },

            _ => (),
        }
    }
}
