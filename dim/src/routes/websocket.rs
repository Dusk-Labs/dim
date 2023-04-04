use std::collections::HashMap;
use std::hash::Hash;
use std::net::SocketAddr;
use std::pin::Pin;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use warp::Filter;

use futures::prelude::*;

use crate::routes;

use dim_web::axum;

pub enum CtrlEvent<A, M>
where
    A: Hash + Eq,
{
    Track {
        addr: A,
        sink: Pin<Box<dyn Sink<WsMessage, Error = WsMessageError> + Send>>,
        auth: Box<database::user::User>,
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

pub type WsMessage = dim_web::axum::extract::ws::Message;

#[derive(Debug)]
pub struct WsMessageError;

impl From<warp::Error> for WsMessageError {
    fn from(value: warp::Error) -> Self {
        Self
    }
}

impl From<axum::Error> for WsMessageError {
    fn from(value: axum::Error) -> Self {
        Self
    }
}

pub async fn handle_websocket_session(
    sink: impl Sink<WsMessage, Error = WsMessageError> + Send + 'static,
    stream: impl Stream<Item = WsMessage>,
    remote_address: Option<SocketAddr>,
    conn: database::DbConnection,
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
                if let Ok(token_data) = database::user::Login::verify_cookie(token) {
                    if let Ok(mut sql_tx) = conn.read().begin().await {
                        if let Ok(u) =
                            database::user::User::get_by_id(&mut sql_tx, token_data).await
                        {
                            let _ = socket_tx.send(CtrlEvent::Track {
                                addr,
                                sink: Box::pin(sink),
                                auth: Box::new(u),
                            });

                            let _ = socket_tx.send(CtrlEvent::SendTo {
                                addr,
                                message: events::Message {
                                    id: -1,
                                    event_type: events::PushEventType::EventAuthOk,
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
                message: events::Message {
                    id: -1,
                    event_type: events::PushEventType::EventAuthErr,
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

fn from_warp_message(inner: warp::ws::Message) -> Result<WsMessage, ()> {
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
            .map(|(code, reason)| dim_web::axum::extract::ws::CloseFrame {
                code: dim_web::axum::extract::ws::CloseCode::from(code),
                reason: std::borrow::Cow::Owned(reason.to_owned()),
            });
        Ok(WsMessage::Close(fr))
    } else {
        Err(())
    }
}

fn from_tungstenite_message(inner: WsMessage) -> Result<warp::ws::Message, WsMessageError> {
    use dim_web::axum::extract::ws::Message;
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

pub fn ws(
    socket_tx: SocketTx,
    conn: database::DbConnection,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::filters::addr::remote())
        .and(routes::global_filters::with_state(socket_tx))
        .and(warp::ws())
        .and(warp::any().map(move || conn.clone()))
        .map(
            |remote_address: Option<SocketAddr>,
             socket_tx: SocketTx,
             ws: warp::ws::Ws,
             conn: database::DbConnection| {
                ws.on_upgrade(move |websocket| async move {
                    let (ws_tx, ws_rx) = websocket.split();
                    let ws_tx = ws_tx
                        .sink_err_into::<WsMessageError>()
                        .with(|m| async move { from_tungstenite_message(m) });

                    let ws_rx = ws_rx
                        .map_err(|_| ())
                        .and_then(|m| async move { from_warp_message(m) })
                        .filter_map(|res| async move { res.ok() });

                    handle_websocket_session(ws_tx, ws_rx, remote_address, conn, socket_tx).await;
                })
            },
        )
}
