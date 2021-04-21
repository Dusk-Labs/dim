use std::{collections::HashMap, fmt::Debug, net::SocketAddr, sync::Arc};

use tokio::{net::TcpListener, runtime::Handle};
use tokio::{
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use futures::prelude::*;
use futures::stream::SplitSink;

#[derive(Debug)]
enum CtrlEvent<M: Debug> {
    Track {
        addr: SocketAddr,
        sink: SplitSink<WebSocketStream<TcpStream>, Message>,
    },

    Forget {
        addr: SocketAddr,
    },

    Send(M),
}

impl<T> CtrlEvent<T>
where
    T: Into<Message> + Clone + Debug,
{
    async fn recv_from_rx(mut rx: UnboundedReceiver<Self>) {
        let mut peers = HashMap::new();
        let mut discard = vec![];

        while let Some(ev) = rx.recv().await {
            discard.clear();

            let _ = match ev {
                CtrlEvent::Track { addr, sink } => peers.insert(addr, sink),
                CtrlEvent::Forget { ref addr } => peers.remove(addr),
                CtrlEvent::Send(body) => {
                    for (addr, sink) in peers.iter_mut() {
                        let result = sink.send(body.clone().into()).await;

                        if result.is_err() {
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

pub async fn serve<A>(
    address: A,
    rt_handle: Handle,
    mut rx: UnboundedReceiver<String>,
) -> std::io::Result<()>
where
    A: tokio::net::ToSocketAddrs,
{
    let listener = TcpListener::bind(address).await?;

    let (i_tx, i_rx) = unbounded_channel::<CtrlEvent<String>>();

    rt_handle.spawn(CtrlEvent::recv_from_rx(i_rx));

    let fut = {
        let i_tx = i_tx.clone();

        async move {
            while let Some(st) = rx.recv().await {
                let _ = i_tx.send(CtrlEvent::Send(st));
            }
        }
    };

    rt_handle.spawn(fut);

    while let Ok((stream, addr)) = listener.accept().await {
        let (stream, i_tx) = match tokio_tungstenite::accept_async(stream).await {
            Ok(stream) => (stream, i_tx.clone()),
            Err(error) => {
                // TODO: log WS errors.
                continue;
            }
        };

        let (out, mut inc) = stream.split();

        let _ = i_tx.send(CtrlEvent::Track {
            addr: addr.clone(),
            sink: out,
        });

        rt_handle.spawn(async move {
            while let Some(Ok(_)) = inc.next().await {} // we don't handle incomming WS messages so just drop'em.
            i_tx.send(CtrlEvent::Forget { addr })
        });
    }

    Ok(())
}
