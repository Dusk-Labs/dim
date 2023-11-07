#![deny(warnings)]

use std::{
    future::IntoFuture,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

pub mod routes;
pub mod tree;

pub use axum;
use axum::{
    extract::{ConnectInfo, State},
    response::{IntoResponse, Response},
    routing,
};
use dim_core::{
    core::EventTx,
    errors::{DimError, ErrorStatusCode},
    routes::dashboard,
    stream_tracking::StreamTracking,
};
use dim_database::DbConnection;
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use nightfall::StateManager;
use tokio::sync::mpsc::UnboundedReceiver;
use warp::Filter;

#[inline]
pub async fn serve(addr: &SocketAddr, router: axum::Router) -> Result<(), hyper::Error> {
    axum::Server::bind(addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
}

struct DimErrorWrapper(DimError);

impl IntoResponse for DimErrorWrapper {
    fn into_response(self) -> Response {
        use DimError as E;

        let status = match self.0 {
            E::LibraryNotFound | E::NoneError | E::NotFoundError | E::ExternalSearchError(_) => {
                StatusCode::NOT_FOUND
            }
            E::StreamingError(_)
            | E::DatabaseError { .. }
            | E::UnknownError
            | E::IOError
            | E::InternalServerError
            | E::UploadFailed
            | E::ScannerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            E::Unauthenticated
            | E::Unauthorized
            | E::InvalidCredentials
            | E::CookieError(_)
            | E::NoToken
            | E::UserNotFound => StatusCode::UNAUTHORIZED,
            E::UsernameNotAvailable => StatusCode::BAD_REQUEST,
            E::UnsupportedFile | E::InvalidMediaType | E::MissingFieldInBody { .. } => {
                StatusCode::NOT_ACCEPTABLE
            }
            E::MediafileRouteError(ref e) => e.status_code(),
        };

        let resp = serde_json::json!({
            "error": serde_json::json!(&self.0)["error"],
            "messsage": self.0.to_string(),
        });
        (status, serde_json::to_string(&resp).unwrap()).into_response()
    }
}

pub async fn warp_core(
    event_tx: EventTx,
    stream_manager: StateManager,
    port: u16,
    event_rx: UnboundedReceiver<String>,
) {
    let state = stream_manager;
    let stream_tracking = StreamTracking::default();
    let conn = dim_database::get_conn()
        .await
        .expect("Failed to grab a handle to the connection pool.");

    macro_rules! warp {
        ($p:path) => {
            ::warp::service($p(conn.clone()))
        };
    }

    let event_repeater = routes::websocket::event_repeater(
        tokio_stream::wrappers::UnboundedReceiverStream::new(event_rx),
        1024,
    );

    let socket_tx = event_repeater.sender();

    tokio::spawn(event_repeater.into_future());

    #[derive(Debug, Clone)]
    struct AppState {
        conn: DbConnection,
        socket_tx: routes::websocket::EventSocketTx,
    }

    async fn ws_handler(
        ws: axum::extract::WebSocketUpgrade,
        ConnectInfo(remote_address): ConnectInfo<SocketAddr>,
        State(AppState { conn, socket_tx }): State<AppState>,
    ) -> Response {
        ws.on_upgrade(move |websocket| async move {
            let (ws_tx, ws_rx) = websocket.split();

            routes::websocket::handle_websocket_session(
                ws_tx.sink_err_into::<routes::websocket::WsMessageError>(),
                ws_rx.filter_map(|m| async move { m.ok() }),
                Some(remote_address),
                conn,
                socket_tx,
            )
            .await;
        })
    }

    let router = axum::Router::new()
        .route(
            "/api/v1/auth/whoami",
            routing::get(routes::auth::whoami).with_state(conn.clone()),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            conn.clone(),
            with_auth,
        ))
        // --- End of routes authenticated by Axum middleware ---
        .route(
            "/api/v1/auth/login",
            routing::post(routes::auth::login).with_state(conn.clone()),
        )
        .route(
            "/api/v1/auth/register",
            routing::post(routes::auth::register).with_state(conn.clone()),
        )
        .route(
            "/api/v1/auth/admin_exists",
            routing::get(routes::auth::admin_exists).with_state(conn.clone()),
        )
        .route(
            "/api/v1/library/:id/media",
            routing::get(routes::library::library_get_media).with_state(conn.clone()),
        )
        .route(
            "/api/v1/library/:id",
            routing::get(routes::library::library_get)
                .post(routes::library::library_post)
                .with_state(conn.clone()),
        )
        .route(
            "/api/v1/library/:id/unmatched",
            routing::get(routes::library::library_get_unmatched).with_state(conn.clone()),
        )
        // .route_service(
        //     "/api/v1/library/*path",
        //     warp::service({
        //         routes::library::filters::library_get_self(conn.clone())
        //             .or(routes::library::filters::get_all_unmatched_media(
        //                 conn.clone(),
        //             ))
        //             .or(routes::library::filters::library_delete(conn.clone()))
        //             .or(routes::library::filters::library_get_self(conn.clone()))
        //             .or(routes::library::filters::get_all_of_library(conn.clone()))
        //     }),
        // )
        // .route_service(
        //     "/api/v1/library",
        //     warp::service(library::filters::library_get(conn.clone()).or(
        //         routes::library::filters::library_post(conn.clone(), event_tx.clone()),
        //     )),
        // )
        .route_service("/api/v1/dashboard", warp!(dashboard::filters::dashboard))
        .route_service(
            "/api/v1/dashboard/banner",
            warp!(dashboard::filters::banners),
        )
        .route_service(
            "/api/v1/search",
            warp!(dim_core::routes::general::filters::search),
        )
        .route_service(
            "/api/v1/filebrowser/",
            warp!(dim_core::routes::general::filters::get_directory_structure),
        )
        .route_service(
            "/api/v1/filebrowser/*path",
            warp!(dim_core::routes::general::filters::get_directory_structure),
        )
        .route_service(
            "/images/*path",
            warp!(dim_core::routes::statik::filters::get_image),
        )
        .route_service(
            "/api/v1/media/*path",
            warp::service({
                dim_core::routes::media::filters::get_media_by_id(conn.clone())
                    .or(dim_core::routes::media::filters::get_media_files(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::media::filters::update_media_by_id(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::media::filters::delete_media_by_id(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::media::filters::tmdb_search(conn.clone()))
                    .or(dim_core::routes::media::filters::map_progress(conn.clone()))
                    .or(dim_core::routes::media::filters::get_mediafile_tree(
                        conn.clone(),
                    ))
                    .or(
                        dim_core::routes::rematch_media::filters::rematch_media_by_id(
                            conn.clone(),
                            event_tx.clone(),
                        ),
                    )
            }),
        )
        .route_service(
            "/api/v1/stream/*path",
            warp::service({
                dim_core::routes::stream::filters::return_virtual_manifest(
                    conn.clone(),
                    state.clone(),
                    stream_tracking.clone(),
                )
                .or(dim_core::routes::stream::filters::return_manifest(
                    conn.clone(),
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(dim_core::routes::stream::filters::get_init(state.clone())
                    .recover(dim_core::routes::global_filters::handle_rejection))
                .or(dim_core::routes::stream::filters::should_client_hard_seek(
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(dim_core::routes::stream::filters::session_get_stderr(
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(dim_core::routes::stream::filters::kill_session(
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(dim_core::routes::stream::filters::get_subtitle(
                    state.clone(),
                ))
                .or(dim_core::routes::stream::filters::get_subtitle_ass(
                    state.clone(),
                ))
                .or(dim_core::routes::stream::filters::get_chunk(state.clone())
                    .recover(dim_core::routes::global_filters::handle_rejection))
            }),
        )
        .route_service(
            "/api/v1/mediafile/*path",
            warp::service({
                dim_core::routes::mediafile::filters::get_mediafile_info(conn.clone()).or(
                    dim_core::routes::mediafile::filters::rematch_mediafile(conn.clone()),
                )
            }),
        )
        .route_service(
            "/api/v1/tv/*path",
            warp!(dim_core::routes::tv::filters::get_tv_seasons),
        )
        .route_service(
            "/api/v1/season/*path",
            warp::service({
                dim_core::routes::tv::filters::patch_episode_by_id(conn.clone())
                    .or(dim_core::routes::tv::filters::delete_season_by_id(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::tv::filters::get_season_episodes(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::tv::filters::patch_episode_by_id(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::tv::filters::delete_episode_by_id(
                        conn.clone(),
                    ))
            }),
        )
        .route_service(
            "/api/v1/episode/*path",
            warp::service({
                dim_core::routes::tv::filters::patch_episode_by_id(conn.clone()).or(
                    dim_core::routes::tv::filters::delete_episode_by_id(conn.clone()),
                )
            }),
        )
        .route_service(
            "/api/v1/user/settings",
            warp::service({
                dim_core::routes::settings::filters::get_user_settings(conn.clone())
                    .or(dim_core::routes::settings::filters::get_user_settings(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::settings::filters::post_user_settings(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::settings::filters::get_global_settings(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::settings::filters::set_global_settings(
                        conn.clone(),
                    ))
            }),
        )
        .route_service(
            "/api/v1/host/settings",
            warp::service({
                dim_core::routes::settings::filters::get_global_settings(conn.clone()).or(
                    dim_core::routes::settings::filters::set_global_settings(conn.clone()),
                )
            }),
        )
        .route_service(
            "/api/v1/user/*path",
            warp::service({
                dim_core::routes::user::filters::change_password(conn.clone())
                    .or(dim_core::routes::user::filters::delete(conn.clone()))
                    .or(dim_core::routes::user::filters::change_username(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::user::filters::upload_avatar(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/auth/*path",
            warp::service({
                dim_core::routes::invites::filters::get_all_invites(conn.clone())
                    .or(dim_core::routes::invites::filters::generate_invite(
                        conn.clone(),
                    ))
                    .or(dim_core::routes::invites::filters::delete_token(
                        conn.clone(),
                    ))
            }),
        )
        .route_service(
            "/",
            warp::service(dim_core::routes::statik::filters::react_routes()),
        )
        .route_service(
            "/*path",
            warp::service(dim_core::routes::statik::filters::react_routes()),
        )
        .route_service(
            "/static/*path",
            warp::service(dim_core::routes::statik::filters::dist_static()),
        )
        .route("/ws", routing::get(ws_handler))
        .with_state(AppState {
            conn: conn.clone(),
            socket_tx,
        })
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer({
            let cors = tower_http::cors::CorsLayer::new()
                // allow requests from any origin
                .allow_origin(tower_http::cors::Any);

            cors
        });

    tracing::info!("Webserver is listening on 0.0.0.0:{}", port);

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let web_fut = serve(&socket_addr, router);

    tokio::select! {
        _ = web_fut => {},
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("CTRL-C received, shutting down...");
            return;
        }
    }
}

pub(crate) async fn with_auth<B>(
    State(conn): State<DbConnection>,
    mut req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, DimErrorWrapper> {
    match req.headers().get(axum::http::header::AUTHORIZATION) {
        Some(token) => {
            let mut tx = match conn.read().begin().await {
                Ok(tx) => tx,
                Err(_) => {
                    return Err(DimErrorWrapper(DimError::DatabaseError {
                        description: String::from("Failed to start transaction"),
                    }))
                }
            };
            let id = dim_database::user::Login::verify_cookie(token.to_str().unwrap().to_string())
                .map_err(|e| DimError::CookieError(e))
                .map_err(|e| DimErrorWrapper(e))?;

            let current_user = dim_database::user::User::get_by_id(&mut tx, id)
                .await
                .map_err(|_| DimError::UserNotFound)
                .map_err(|e| DimErrorWrapper(e))?;

            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        }
        None => Err(DimErrorWrapper(DimError::NoToken)),
    }
}
