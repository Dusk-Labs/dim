use std::collections::HashMap;
use std::future::IntoFuture;
use std::hash::Hash;
use std::net::SocketAddr;
use std::pin::Pin;

use futures::future::BoxFuture;
use tokio::sync::mpsc;

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

async fn ctrl_event_processor<A, T>(mut rx: mpsc::Receiver<CtrlEvent<A, T>>)
where
    A: Hash + Eq + Clone,
    T: ToOwned<Owned = String> + Send,
{
    let mut peers = HashMap::new();
    let mut discard = vec![];

    loop {
        for addr in &discard {
            let _ = peers.remove(addr);
        }

        discard.clear();

        let Some(ev) = rx.recv().await else { break };

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
    socket_tx: EventSocketTx,
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
                            }).await;

                            let _ = socket_tx.send(CtrlEvent::SendTo {
                                addr,
                                message: dim_events::Message {
                                    id: -1,
                                    event_type: dim_events::PushEventType::EventAuthOk,
                                }
                                .to_string(),
                            }).await;

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
            }).await;
        }
    }

    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                break;
            }

            None = stream.next() => {
                let _ = socket_tx.send(CtrlEvent::Forget { addr }).await;
                break;
            }
        }
    }
}

pub type EventSocketTx<A = SocketAddr, M = String> = mpsc::Sender<CtrlEvent<A, M>>;

#[derive(Debug)]
#[must_use]
pub struct EventRepeater<StreamFut, Fut, T> {
    stream_forward_fut: StreamFut,
    fut: Fut,
    tx: T,
}

impl<StreamFut, Fut, T> EventRepeater<StreamFut, Fut, T> {
    fn new(stream_forward_fut: StreamFut, fut: Fut, tx: T) -> Self {
        Self {
            stream_forward_fut,
            fut,
            tx,
        }
    }
}

impl<StreamFut, Fut, A, M> EventRepeater<StreamFut, Fut, EventSocketTx<A, M>>
where
    A: Hash + Eq + Clone + Send + Sync + 'static,
    M: ToOwned<Owned = String> + Send + Sync + 'static,
{
    #[inline]
    pub fn sender(&self) -> EventSocketTx<A, M> {
        self.tx.clone()
    }
}

impl<T, U, A, M> IntoFuture for EventRepeater<T, U, EventSocketTx<A, M>>
where
    T: Future + Send + 'static,
    T::Output: Send,
    U: Future + Send + 'static,
    U::Output: Send,
    A: Hash + Eq + Clone + Send + Sync + 'static,
    M: ToOwned<Owned = String> + Send + Sync + 'static,
{
    type Output = ();

    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        async move {
            tokio::join!(self.stream_forward_fut, self.fut);
        }
        .boxed()
    }
}

pub fn event_repeater<S, T, A, M>(
    source: S,
    capacity: usize,
) -> EventRepeater<
    impl Future<Output = ()> + Send + 'static,
    impl Future<Output = ()> + Send + 'static,
    EventSocketTx<A, M>,
>
where
    S: Stream<Item = T> + Send + 'static,
    T: IntoCtrlEvent<A, M> + Send,
    A: Hash + Eq + Clone + Send + Sync + 'static,
    M: ToOwned<Owned = String> + Send + Sync + 'static,
{
    let (tx, rx) = mpsc::channel::<CtrlEvent<A, M>>(capacity);

    let ctrl_event_processor_fut = ctrl_event_processor::<A, M>(rx);

    let stream_forward_tx = tx.clone();
    let stream_forward = async move {
        tokio::pin!(source);
        while let Some(t) = source.next().await {
            if stream_forward_tx.send(t.into_ctrl_event()).await.is_err() {
                break;
            };
        }
    };

    EventRepeater::new(stream_forward, ctrl_event_processor_fut, tx)
}
