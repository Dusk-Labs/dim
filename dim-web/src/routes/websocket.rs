use std::collections::HashMap;
use std::hash::Hash;
use std::net::SocketAddr;
use std::pin::Pin;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use futures::prelude::*;

pub enum CtrlEvent<A, M>
where
    A: Hash + Eq,
{
    Track {
        addr: A,
        sink: Pin<Box<dyn Sink<WsMessage, Error = WsMessageError> + Send>>,
        auth: Box<dim_database::user::User>,
    },

    Forget {
        addr: A,
    },

    SendTo {
        addr: A,
        message: M,
    },

    SendAll(M),
}

pub trait IntoCtrlEvent<A, M>: Sync + Send + Clone + 'static
where
    A: Hash + Eq,
{
    fn into_ctrl_event(self) -> CtrlEvent<A, M>;
}

impl<A> IntoCtrlEvent<A, String> for String
where
    A: Hash + Eq,
{
    fn into_ctrl_event(self) -> CtrlEvent<A, String> {
        CtrlEvent::SendAll(self)
    }
}

async fn ctrl_event_processor<A, T>(mut rx: UnboundedReceiver<CtrlEvent<A, T>>)
where
    A: Hash + Eq + Clone,
    T: ToOwned<Owned = String> + Send,
{
    let mut peers = HashMap::new();
    let mut discard = vec![];

    while let Some(ev) = rx.recv().await {
        for addr in &discard {
            let _ = peers.remove(addr);
        }

        discard.clear();

        match ev {
            CtrlEvent::Track { addr, sink, auth } => {
                peers.insert(addr, (sink, auth));
            }

            CtrlEvent::Forget { ref addr } => {
                peers.remove(addr);
            }

            CtrlEvent::SendAll(body) => {
                for (addr, (sink, _)) in peers.iter_mut() {
                    let result = sink.send(WsMessage::Text(body.to_owned())).await;

                    if result.is_err() {
                        let _ = sink.close().await;
                        discard.push(addr.clone());
                    }
                }
            }

            CtrlEvent::SendTo { addr, message } => {
                if let Some((sink, _)) = peers.get_mut(&addr) {
                    let result = sink.send(WsMessage::Text(message.to_owned())).await;

                    if result.is_err() {
                        let _ = sink.close().await;
                        discard.push(addr.clone());
                    }
                }
            }
        };
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ClientActions {
    Authenticate { token: String },
}

pub type WsMessage = axum::extract::ws::Message;

#[derive(Debug)]
pub struct WsMessageError;

impl From<warp::Error> for WsMessageError {
    fn from(_: warp::Error) -> Self {
        Self
    }
}

impl From<axum::Error> for WsMessageError {
    fn from(_: axum::Error) -> Self {
        Self
    }
}

pub async fn handle_websocket_session(
    sink: impl Sink<WsMessage, Error = WsMessageError> + Send + 'static,
    stream: impl Stream<Item = WsMessage>,
    remote_address: Option<SocketAddr>,
    conn: dim_database::DbConnection,
    socket_tx: SocketTx,
) {
    let addr = match remote_address {
        Some(addr) => addr,
        None => return,
    };

    tokio::pin!(stream);

    'auth_loop: while let Some(message) = stream.next().await {
        if let WsMessage::Text(st) = message {
            if let Ok(ClientActions::Authenticate { token }) = serde_json::from_str(&st) {
                if let Ok(token_data) = dim_database::user::Login::verify_cookie(token) {
                    if let Ok(mut sql_tx) = conn.read().begin().await {
                        if let Ok(u) =
                            dim_database::user::User::get_by_id(&mut sql_tx, token_data).await
                        {
                            let _ = socket_tx.send(CtrlEvent::Track {
                                addr,
                                sink: Box::pin(sink),
                                auth: Box::new(u),
                            });

                            let _ = socket_tx.send(CtrlEvent::SendTo {
                                addr,
                                message: dim_events::Message {
                                    id: -1,
                                    event_type: dim_events::PushEventType::EventAuthOk,
                                }
                                .to_string(),
                            });

                            break 'auth_loop;
                        }
                    }
                }
            }

            let _ = socket_tx.send(CtrlEvent::SendTo {
                addr,
                message: dim_events::Message {
                    id: -1,
                    event_type: dim_events::PushEventType::EventAuthErr,
                }
                .to_string(),
            });
        }
    }

    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                break;
            }

            None = stream.next() => {
                let _ = socket_tx.send(CtrlEvent::Forget { addr });
                break;
            }
        }
    }
}

pub fn from_warp_message(inner: warp::ws::Message) -> Result<WsMessage, ()> {
    if inner.is_binary() {
        Ok(WsMessage::Binary(inner.as_bytes().into()))
    } else if inner.is_text() {
        Ok(WsMessage::Text(
            String::from_utf8_lossy(inner.as_bytes()).to_string(),
        ))
    } else if inner.is_ping() {
        Ok(WsMessage::Ping(inner.as_bytes().into()))
    } else if inner.is_pong() {
        Ok(WsMessage::Pong(inner.as_bytes().into()))
    } else if inner.is_close() {
        let fr = inner
            .close_frame()
            .map(|(code, reason)| axum::extract::ws::CloseFrame {
                code: axum::extract::ws::CloseCode::from(code),
                reason: std::borrow::Cow::Owned(reason.to_owned()),
            });
        Ok(WsMessage::Close(fr))
    } else {
        Err(())
    }
}

pub fn from_tungstenite_message(inner: WsMessage) -> Result<warp::ws::Message, WsMessageError> {
    use axum::extract::ws::Message;
    use warp::ws;

    let m = match inner {
        Message::Text(st) => ws::Message::text(st),
        Message::Binary(bin) => ws::Message::binary(bin),
        Message::Ping(v) => ws::Message::ping(v),
        Message::Pong(v) => ws::Message::pong(v),
        Message::Close(Some(frame)) => ws::Message::close_with(frame.code, frame.reason),
        Message::Close(None) => ws::Message::close(),
    };

    Ok(m)
}

pub type SocketTx<A = SocketAddr, M = String> = UnboundedSender<CtrlEvent<A, M>>;

pub fn event_repeater<S, T, A, M>(
    source: S,
) -> (
    impl Future<Output = ()> + Send + 'static,
    impl Future<Output = ()> + Send + 'static,
    SocketTx<A, M>,
)
where
    S: Stream<Item = T> + Send + 'static,
    T: IntoCtrlEvent<A, M> + Send,
    A: Hash + Eq + Clone + Send + Sync + 'static,
    M: ToOwned<Owned = String> + Send + Sync + 'static,
{
    let (tx, rx) = unbounded_channel::<CtrlEvent<A, M>>();

    let event_prcoessor = ctrl_event_processor::<A, M>(rx);
    let stream_forward_tx = tx.clone();
    let stream_forward = async move {
        tokio::pin!(source);
        while let Some(t) = source.next().await {
            if stream_forward_tx.send(t.into_ctrl_event()).is_err() {
                break;
            };
        }
    };

    (stream_forward, event_prcoessor, tx)
}
