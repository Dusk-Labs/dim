use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::hash::Hash;
use std::io;
use std::{fmt::Debug, net::SocketAddr, sync::Arc};

use tokio::net::ToSocketAddrs;
use tokio::{net::TcpListener, runtime::Handle};
use tokio::{
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use futures::prelude::*;
use futures::stream::{SplitSink, SplitStream};
use xtra::Actor;
use xtra::Address;

#[derive(Debug)]
enum CtrlEvent<A, M>
where
    A: Hash + Eq,
    M: Debug,
{
    Track {
        addr: A,
        sink: SplitSink<WebSocketStream<TcpStream>, Message>,
    },

    Forget {
        addr: A,
    },

    Send(M),
}

impl<A, T> CtrlEvent<A, T>
where
    T: Into<Message> + Clone + Debug,
    A: Hash + Eq + Clone,
{
    async fn recv_from_rx(mut rx: UnboundedReceiver<Self>) {
        let mut peers = HashMap::new();
        let mut discard = vec![];

        while let Some(ev) = rx.recv().await {
            discard.clear();

            match ev {
                CtrlEvent::Track { addr, sink } => {
                    peers.insert(addr, sink);
                }

                CtrlEvent::Forget { ref addr } => {
                    peers.remove(addr);
                }

                CtrlEvent::Send(body) => {
                    for (addr, sink) in peers.iter_mut() {
                        let result = sink.send(body.clone().into()).await;

                        if result.is_err() {
                            let _ = sink.close().await;
                            discard.push(addr.clone());
                        }
                    }

                    for addr in &discard {
                        let _ = peers.remove(addr);
                    }

                    continue;
                }
            };
        }
    }
}

#[async_trait::async_trait]
pub(crate) trait WebsocketServer {
    async fn bind<S>(&mut self, address: S) -> io::Result<TcpListener>
    where
        S: ToSocketAddrs + Send;

    async fn on_message(&mut self, addr: SocketAddr, message: Message) {}

    async fn on_connect(
        &mut self,
        addr: SocketAddr,
        stream: WebSocketStream<TcpStream>,
    ) -> Result<WebSocketStream<TcpStream>, ()> {
        Ok(stream)
    }

    async fn serve<S, M>(
        &mut self,
        address: S,
        rt_handle: Handle,
        mut event_rx: UnboundedReceiver<M>,
    ) -> io::Result<()>
    where
        M: Sync + Send + fmt::Debug + Into<Message> + Clone + 'static,
        S: ToSocketAddrs + Send,
    {
        let listener = self.bind(address).await?;

        let (i_tx, i_rx) = unbounded_channel::<CtrlEvent<SocketAddr, M>>();

        rt_handle.spawn(CtrlEvent::recv_from_rx(i_rx));

        let forwarder_fut = {
            let i_tx = i_tx.clone();

            async move {
                while let Some(st) = event_rx.recv().await {
                    let _ = i_tx.send(CtrlEvent::Send(st));
                }
            }
        };

        rt_handle.spawn(forwarder_fut);

        let (m_tx, mut m_rx) = unbounded_channel::<(SocketAddr, Message)>();

        'outer: loop {
            tokio::select! {
                biased;

                incoming = listener.accept() => {
                    let (stream, addr) = match incoming {
                        Ok(sa) => sa,
                        Err(_) => break 'outer,
                    };

                    let (stream, i_tx) = match tokio_tungstenite::accept_async(stream).await {
                        Ok(stream) => (stream, i_tx.clone()),
                        Err(error) => {
                            // TODO: log WS errors.
                            continue 'outer;
                        }
                    };

                    let (out, mut inc) = match self.on_connect(addr.clone(), stream).await {
                        Ok(stream) => stream.split(),
                        Err(_) => continue 'outer,
                    };

                    let _ = i_tx.send(CtrlEvent::Track {
                        addr: addr.clone(),
                        sink: out,
                    });

                    let m_tx = m_tx.clone();

                    rt_handle.spawn(async move {
                        while let Some(Ok(message)) = inc.next().await {
                            if let Err(_) = m_tx.send((addr, message)) {
                                break;
                            }
                        }

                        i_tx.send(CtrlEvent::Forget { addr })
                    });
                }

                message = m_rx.recv() => {
                    let (addr, message) = match message {
                        Some(p) => p,
                        None => break 'outer,
                    };

                    self.on_message(addr, message).await;
                }

                else => break 'outer,
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl WebsocketServer for Option<std::net::TcpListener> {
    async fn bind<S>(&mut self, address: S) -> io::Result<TcpListener>
    where
        S: ToSocketAddrs + Send,
    {
        TcpListener::try_from(self.take().unwrap())
    }
}

pub async fn serve<S, M>(
    address: S,
    rt_handle: Handle,
    mut event_rx: UnboundedReceiver<M>,
) -> std::io::Result<()>
where
    M: Sync + Send + fmt::Debug + Into<Message> + Clone + 'static,
    S: std::net::ToSocketAddrs
{
    let listener = std::net::TcpListener::bind(address)?;

    Some(listener).serve("<already bound>", rt_handle, event_rx).await
}
