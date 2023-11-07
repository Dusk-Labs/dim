#![deny(warnings)]

use std::future::IntoFuture;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod routes;
pub mod tree;

pub use axum;
use axum::extract::{ConnectInfo, State};
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;

use dim_core::core::EventTx;
use dim_core::routes::dashboard;
use dim_core::stream_tracking::StreamTracking;
use dim_database::DbConnection;

use futures::{Future, SinkExt, StreamExt};
use nightfall::StateManager;
use tokio::sync::mpsc::UnboundedReceiver;
use warp::Filter;

pub mod error;
pub use error::DimErrorWrapper;

pub mod middleware;
pub use middleware::verify_cookie_token;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DbConnection,
    socket_tx: routes::websocket::EventSocketTx,
    event_tx: EventTx,
    state: StateManager,
    stream_tracking: StreamTracking,
}

fn library_routes(app: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "api/v1/library",
            post(routes::library::library_post).get(routes::library::library_get_all),
        )
        .route(
            "/api/v1/library/:id/media",
            get(routes::library::library_get_media).with_state(app.conn.clone()),
        )
        .route(
            "/api/v1/library/:id",
            get(routes::library::library_get_one).delete(routes::library::library_delete),
        )
        .route(
            "/api/v1/library/:id/unmatched",
            get(routes::library::library_get_unmatched).with_state(app.conn.clone()),
        )
}

fn media_routes(AppState { conn, event_tx, .. }: AppState) -> Router<AppState> {
    Router::new().route_service(
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
}

fn stream_routes(
    AppState {
        conn,
        state,
        stream_tracking,
        ..
    }: AppState,
) -> Router<AppState> {
    Router::new().route_service(
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
}

fn season_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new().route_service(
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
}

fn settings_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new().route_service(
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
}

pub async fn start_webserver(
    event_tx: EventTx,
    stream_manager: StateManager,
    port: u16,
    event_rx: UnboundedReceiver<String>,
    shutdown_fut: impl Future<Output = ()> + Send + 'static,
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

    async fn ws_handler(
        ws: axum::extract::WebSocketUpgrade,
        ConnectInfo(remote_address): ConnectInfo<SocketAddr>,
        State(AppState {
            conn, socket_tx, ..
        }): State<AppState>,
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

    let app = AppState {
        conn: conn.clone(),
        socket_tx: socket_tx.clone(),
        event_tx: event_tx.clone(),
        state,
        stream_tracking,
    };

    let router = axum::Router::new()
        .route(
            "/api/v1/auth/whoami",
            get(routes::auth::whoami).with_state(conn.clone()),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            conn.clone(),
            verify_cookie_token,
        ))
        // --- End of routes authenticated by Axum middleware ---
        .route(
            "/api/v1/auth/login",
            post(routes::auth::login).with_state(conn.clone()),
        )
        .route(
            "/api/v1/auth/register",
            post(routes::auth::register).with_state(conn.clone()),
        )
        .route(
            "/api/v1/auth/admin_exists",
            get(routes::auth::admin_exists).with_state(conn.clone()),
        )
        .merge(library_routes(app.clone()))
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
        .merge(media_routes(app.clone()))
        .merge(stream_routes(app.clone()))
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
        .merge(season_routes(app.clone()))
        .route_service(
            "/api/v1/episode/*path",
            warp::service({
                dim_core::routes::tv::filters::patch_episode_by_id(conn.clone()).or(
                    dim_core::routes::tv::filters::delete_episode_by_id(conn.clone()),
                )
            }),
        )
        .merge(settings_routes(app.clone()))
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
        .route("/ws", get(ws_handler))
        .with_state(app)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer({
            let cors = tower_http::cors::CorsLayer::new()
                // allow requests from any origin
                .allow_origin(tower_http::cors::Any);

            cors
        });

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    tracing::info!(%addr, "webserver is listening");

    let web_fut =
        axum::Server::bind(&addr).serve(router.into_make_service_with_connect_info::<SocketAddr>());

    tokio::select! {
        _ = web_fut => {},
        _ = shutdown_fut => {
            return;
        }
    }
}
