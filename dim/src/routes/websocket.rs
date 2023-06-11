use std::net::SocketAddr;

use dim_web::routes::websocket::{handle_websocket_session, EventSocketTx, WsMessageError};

use warp::Filter;

use futures::prelude::*;

use crate::routes;

pub fn ws(
    socket_tx: EventSocketTx,
    conn: dim_database::DbConnection,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::filters::addr::remote())
        .and(routes::global_filters::with_state(socket_tx))
        .and(warp::ws())
        .and(warp::any().map(move || conn.clone()))
        .map(
            |remote_address: Option<SocketAddr>,
             socket_tx: EventSocketTx,
             ws: warp::ws::Ws,
             conn: dim_database::DbConnection| {
                ws.on_upgrade(move |websocket| async move {
                    let (ws_tx, ws_rx) = websocket.split();
                    let ws_tx = ws_tx
                        .sink_err_into::<WsMessageError>()
                        .with(|m| async move {
                            dim_web::routes::websocket::from_tungstenite_message(m)
                        });

                    let ws_rx = ws_rx
                        .map_err(|_| ())
                        .and_then(
                            |m| async move { dim_web::routes::websocket::from_warp_message(m) },
                        )
                        .filter_map(|res| async move { res.ok() });

                    handle_websocket_session(ws_tx, ws_rx, remote_address, conn, socket_tx).await;
                })
            },
        )
}
