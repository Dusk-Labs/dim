use std::future::IntoFuture;
use std::net::SocketAddr;

pub mod routes;
pub mod tree;

pub use axum;
use axum::extract::ConnectInfo;
use axum::extract::DefaultBodyLimit;
use axum::extract::State;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::patch;
use axum::routing::post;
use axum::Router;

use dim_core::core::EventTx;
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
            "/api/v1/library",
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

fn auth_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
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
}

fn dashboard_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/dashboard",
            get(routes::dashboard::dashboard).with_state(conn.clone()),
        )
        .route(
            "/api/v1/dashboard/banner",
            get(routes::dashboard::banners).with_state(conn.clone()),
        )
}

fn media_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/media/:id",
            get(routes::media::get_media_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/:id/files",
            get(routes::media::get_media_files).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/:id/tree",
            get(routes::media::get_mediafile_tree).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/:id",
            patch(routes::media::update_media_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/:id",
            delete(routes::media::delete_media_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/tmdb_search",
            get(routes::media::tmdb_search).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/:id/progress",
            post(routes::media::map_progress).with_state(conn.clone()),
        )
        .route(
            "/api/v1/media/:id/match",
            patch(routes::media::rematch_media_by_id).with_state(conn.clone()),
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

fn episode_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/episode/:id",
            patch(routes::tv::patch_episode_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/episode/:id",
            delete(routes::tv::delete_episode_by_id).with_state(conn.clone()),
        )
}

fn season_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/season/:id",
            get(routes::tv::get_season_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/season/:id",
            patch(routes::tv::patch_season_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/season/:id",
            delete(routes::tv::delete_season_by_id).with_state(conn.clone()),
        )
        .route(
            "/api/v1/season/:id/episodes",
            get(routes::tv::get_season_episodes).with_state(conn.clone()),
        )
}

fn tv_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/tv/:id/season",
            get(routes::tv::get_tv_seasons).with_state(conn.clone()),
        )
}

fn filebrowser_routes(AppState { .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/filebrowser/",
            get(routes::filebrowser::get_directory_structure),
        )
        .route(
            "/api/v1/filebrowser/*path",
            get(routes::filebrowser::get_directory_structure),
        )
}

fn mediafile_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/mediafile/:id",
            get(routes::mediafile::get_mediafile_info).with_state(conn.clone()),
        )
        .route(
            "/api/v1/mediafile/match",
            patch(routes::mediafile::rematch_mediafile).with_state(conn.clone()),
        )
}

fn settings_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/user/settings",
            get(routes::settings::get_user_settings).with_state(conn.clone()),
        )
        .route(
            "/api/v1/user/settings",
            post(routes::settings::post_user_settings).with_state(conn.clone()),
        )
        .route(
            "/api/v1/host/settings",
            get(routes::settings::http_get_global_settings).with_state(conn.clone()),
        )
        .route(
            "/api/v1/host/settings",
            post(routes::settings::http_set_global_settings).with_state(conn.clone()),
        )
}

fn user_routes(AppState { conn, .. }: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/user/password",
            patch(routes::user::change_password).with_state(conn.clone()),
        )
        .route(
            "/api/v1/user/delete",
            delete(routes::user::delete).with_state(conn.clone()),
        )
        .route(
            "/api/v1/user/username",
            patch(routes::user::change_username).with_state(conn.clone()),
        )
        .route(
            "/api/v1/user/avatar",
            post(routes::user::upload_avatar).with_state(conn.clone()),
        ).layer(DefaultBodyLimit::max(5_000_000))
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
        .route(
            "/api/v1/auth/invites",
            get(routes::auth::get_all_invites).with_state(conn.clone()),
        )
        .route(
            "/api/v1/auth/new_invite",
            post(routes::auth::generate_invite).with_state(conn.clone()),
        )
        .route(
            "/api/v1/auth/token/:token",
            delete(routes::auth::delete_token).with_state(conn.clone()),
        )
        .merge(dashboard_routes(app.clone()))
        .merge(episode_routes(app.clone()))
        .merge(library_routes(app.clone()))
        .merge(media_routes(app.clone()))
        .merge(mediafile_routes(app.clone()))
        .merge(season_routes(app.clone()))
        .merge(tv_routes(app.clone()))
        .merge(filebrowser_routes(app.clone()))
        .merge(user_routes(app.clone()))
        .route(
            "/api/v1/search",
            get(routes::search::search).with_state(conn.clone()),
        )
        .merge(settings_routes(app.clone()))
        .route_layer(axum::middleware::from_fn_with_state(
            conn.clone(),
            verify_cookie_token,
        ))
        // --- End of routes authenticated by Axum middleware ---
        .merge(auth_routes(app.clone()))
        .route_service(
            "/images/*path",
            warp!(dim_core::routes::statik::filters::get_image),
        )
        .merge(stream_routes(app.clone()))
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
