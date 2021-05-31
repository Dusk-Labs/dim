use std::{io, time::Duration};

use futures::{SinkExt, StreamExt};
use rocket::figment::providers::Env;
use slog::{debug, trace};
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::mpsc::UnboundedSender,
};
use tokio_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};
use xtra::Address;

use crate::websocket::WebsocketServer;

use super::runner::{Envelope, PluginMessage, PluginServer, RunnerQuery, ServerState};

pub struct PluginHost {
    pub(super) listener: Option<TcpListener>,
    pub(super) inner: Address<PluginServer>,
    pub(super) logger: slog::Logger,
    pub(super) out_tx: UnboundedSender<Envelope>,
}

#[async_trait::async_trait]
impl WebsocketServer for PluginHost {
    async fn bind<S>(&mut self, address: S) -> io::Result<TcpListener>
    where
        S: ToSocketAddrs + Send,
    {
        // Avoids a race condition when we start the server before spawning all of the plugins
        // but the server is unaware of the token the plugin will use to authenticate so the server
        // might accidentally ignore a plugin who manages to connect quicker than we can register the token.
        loop {
            let (tx, mut rx) = tokio::sync::oneshot::channel();

            self.inner
                .send(RunnerQuery::GetServerState { tx })
                .await
                .unwrap();

            if let Ok(ServerState::Running) = rx.try_recv() {
                break;
            } else {
                // probably enough time for all of the plugins to be spawned.
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        }

        match self.listener.take() {
            Some(listener) => Ok(listener),
            None => TcpListener::bind(address).await,
        }
    }

    async fn on_message(&mut self, addr: std::net::SocketAddr, message: tungstenite::Message) {
        let st = match message {
            tungstenite::Message::Text(st) => st,
            _ => return,
        };

        let envelope = match serde_json::from_str(&st) {
            Ok(it) => Envelope(addr, it),
            _ => return,
        };

        debug!(self.logger, "Plugin: {:?}", envelope);

        let inner = self.inner.clone();
        let out_tx = self.out_tx.clone();

        let fut = async move {
            if let Err(resp) = inner.send(envelope).await.unwrap() {
                out_tx.send(Envelope(addr, resp));
            }
        };

        tokio::runtime::Handle::current().spawn(fut);
    }

    async fn on_connect(
        &mut self,
        addr: std::net::SocketAddr,
        mut stream: WebSocketStream<TcpStream>,
    ) -> Result<WebSocketStream<TcpStream>, ()> {
        debug!(self.logger, "Plugin connect: {:?}", addr);

        let auth = async {
            let mut result = None;

            while let Some(Ok(Message::Text(st))) = stream.next().await {
                let message = match serde_json::from_str(&st) {
                    Ok(it) => it,
                    _ => continue,
                };

                let PluginMessage { trigger, data, .. } = &message;

                if let ("AUTH", serde_json::Value::String(_)) = (trigger.as_str(), data) {
                    let ev = Envelope(addr, message.clone());

                    if let Ok(_) = self.inner.send(ev).await {
                        result = Some(stream);
                    }

                    break;
                } else {
                    let err = serde_json::to_string(
                        &json!({"t": "ERROR", "d": "You must authenticate first."}),
                    )
                    .unwrap();

                    stream.send(Message::Text(err)).await;
                }
            }

            result
        };

        if let Some(stream) = tokio::time::timeout(Duration::from_millis(10), auth)
            .await
            .ok()
            .flatten()
        {
            Ok(stream)
        } else {
            debug!(
                self.logger,
                "Plugin auth timed out. Rejecting... {:?}", addr
            );
            Err(())
        }
    }
}
