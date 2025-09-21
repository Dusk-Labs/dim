#![deny(warnings)]

use std::future::IntoFuture;
use std::net::SocketAddr;

pub mod routes;
pub mod tree;

pub use axum;
use axum::extract::{ConnectInfo, State};
use axum::response::Response;
use axum::routing::{delete, get, patch, post};
use axum::Router;

use dim_core::core::EventTx;
use dim_core::stream_tracking::StreamTracking;
use dim_database::DbConnection;

use futures::{Future, SinkExt, StreamExt};
use nightfall::StateManager;
use tokio::sync::mpsc::UnboundedReceiver;

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

fn library_routes(_app: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/library",
            post(routes::library::library_post).get(routes::library::library_get_all),
        )
        .route(
            "/api/v1/library/:id/media",
            get(routes::library::library_get_media),
        )
        .route(
            "/api/v1/library/:id",
            get(routes::library::library_get_one).delete(routes::library::library_delete),
        )
        .route(
            "/api/v1/library/:id/unmatched",
            get(routes::library::library_get_unmatched),
        )
}

fn auth_routes(AppState { .. }: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/login", post(routes::auth::login))
        .route("/api/v1/auth/register", post(routes::auth::register))
        .route("/api/v1/auth/admin_exists", get(routes::auth::admin_exists))
}

fn media_routes(AppState { .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/media/:id",
            get(routes::media::get_media_by_id)
                .patch(routes::media::update_media_by_id)
                .delete(routes::media::delete_media_by_id),
        )
        .route(
            "/api/v1/media/:id/files",
            get(routes::media::get_media_files),
        )
        .route(
            "/api/v1/media/:id/tree",
            get(routes::media::get_mediafile_tree),
        )
        .route(
            "/api/v1/media/:id/progress",
            post(routes::media::map_progress),
        )
        .route("/api/v1/media/tmdb_search", get(routes::media::tmdb_search))
        .route(
            "/api/v1/media/:id/rematch",
            post(routes::media::rematch_media_by_id),
        )
}

fn stream_routes(
    AppState {
        conn: _conn,
        state: _state,
        stream_tracking: _stream_tracking,
        ..
    }: AppState,
) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/stream/:id/manifest",
            get(routes::stream::return_virtual_manifest),
        )
        .route(
            "/api/v1/stream/:gid/manifest.mpd",
            get(routes::stream::return_manifest),
        )
        .route(
            "/api/v1/stream/:id/data/init.mp4",
            get(routes::stream::get_init),
        )
        .route(
            "/api/v1/stream/:id/data/*chunk",
            get(routes::stream::get_chunk),
        )
        .route(
            "/api/v1/stream/:id/data/stream.vtt",
            get(routes::stream::get_subtitle),
        )
        .route(
            "/api/v1/stream/:id/data/stream.ass",
            get(routes::stream::get_subtitle_ass),
        )
        .route(
            "/api/v1/stream/:gid/state/should_hard_seek/:chunk_num",
            get(routes::stream::should_client_hard_seek),
        )
        .route(
            "/api/v1/stream/:gid/state/get_stderr",
            get(routes::stream::session_get_stderr),
        )
        .route(
            "/api/v1/stream/:gid/state/kill",
            delete(routes::stream::kill_session),
        )
}

fn season_routes(_app: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/season/:id",
            get(routes::tv::get_season_by_id)
                .patch(routes::tv::patch_season_by_id)
                .delete(routes::tv::delete_season_by_id),
        )
        .route(
            "/api/v1/season/:id/episodes",
            get(routes::tv::get_season_episodes),
        )
        .route(
            "/api/v1/season/:id/episode/:episode_id",
            patch(routes::tv::patch_episode_by_id).delete(routes::tv::delete_episode_by_id),
        )
}

fn settings_routes(AppState { .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/user/settings",
            get(|| async { "User settings - needs actual implementation" }),
        )
        .route(
            "/api/v1/user/settings",
            post(|| async { "User settings post - needs actual implementation" }),
        )
}

pub async fn start_webserver(
    address: SocketAddr,
    event_tx: EventTx,
    stream_manager: StateManager,
    event_rx: UnboundedReceiver<String>,
    shutdown_fut: impl Future<Output = ()> + Send + 'static,
) {
    let state = stream_manager;
    let stream_tracking = StreamTracking::default();
    let conn = dim_database::get_conn()
        .await
        .expect("Failed to grab a handle to the connection pool.");

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
        .route("/api/v1/auth/whoami", get(routes::auth::whoami))
        .route_layer(axum::middleware::from_fn_with_state(
            conn.clone(),
            verify_cookie_token,
        ))
        // --- End of routes authenticated by Axum middleware ---
        .merge(auth_routes(app.clone()))
        .merge(library_routes(app.clone()))
        .route("/api/v1/dashboard", get(routes::dashboard::dashboard))
        .route("/api/v1/dashboard/banner", get(routes::dashboard::banners))
        .route("/api/v1/search", get(routes::search::search))
        .route(
            "/api/v1/filebrowser/*path",
            get(routes::filebrowser::get_directory_structure),
        )
        .route("/images/*path", get(routes::statik::get_image))
        .merge(media_routes(app.clone()))
        .merge(stream_routes(app.clone()))
        .route(
            "/api/v1/mediafile/:id",
            get(routes::mediafile::get_mediafile_info),
        )
        .route(
            "/api/v1/mediafile/match",
            patch(routes::mediafile::rematch_mediafile),
        )
        .route("/api/v1/tv/:id/season", get(routes::tv::get_tv_seasons))
        .merge(season_routes(app.clone()))
        .route(
            "/api/v1/episode/:id",
            patch(routes::tv::patch_episode_by_id).delete(routes::tv::delete_episode_by_id),
        )
        .merge(settings_routes(app.clone()))
        .route(
            "/api/v1/host/settings",
            get(routes::settings::http_get_global_settings)
                .post(routes::settings::http_set_global_settings),
        )
        .route("/api/v1/user/password", post(routes::user::change_password))
        .route("/api/v1/user", delete(routes::user::delete))
        .route("/api/v1/username", post(routes::user::change_username))
        .route("/api/v1/user/avatar", post(routes::user::upload_avatar))
        .route(
            "/api/v1/auth/invites",
            get(routes::auth::get_all_invites).post(routes::auth::generate_invite),
        )
        .route(
            "/api/v1/auth/token/:token",
            delete(routes::auth::delete_token),
        )
        .route("/", get(routes::statik::react_routes))
        .route("/*path", get(routes::statik::react_routes))
        .route("/static/*path", get(routes::statik::dist_static))
        .route("/ws", get(ws_handler))
        .with_state(app)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer({
            let cors = tower_http::cors::CorsLayer::new()
                // allow requests from any origin
                .allow_origin(tower_http::cors::Any);

            cors
        });

    tracing::info!(%address, "webserver is listening");

    let web_fut = axum::Server::bind(&address)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>());

    tokio::select! {
        _ = web_fut => {},
        _ = shutdown_fut => {
            return;
        }
    }
}
