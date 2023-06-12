use crate::routes;
use crate::routes::*;
use crate::scanner;
use crate::stream_tracking::StreamTracking;

use dim_database::library::MediaType;
use dim_extern_api::tmdb::TMDBMetadataProvider;

use dim_web::axum::extract::ConnectInfo;
use dim_web::axum::extract::State;
use futures::SinkExt;
use futures::StreamExt;
use once_cell::sync::OnceCell;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use dim_web::routes::websocket;

use warp::Filter;

use std::future::IntoFuture;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;

pub type StateManager = nightfall::StateManager;
pub type DbConnection = dim_database::DbConnection;
pub type EventTx = UnboundedSender<String>;

/// Path to where metadata is stored and should be fetched to.
pub static METADATA_PATH: OnceCell<String> = OnceCell::new();

/// Function dumps a list of all libraries in the database and starts a scanner for each which
/// monitors for new files using fsnotify. It also scans all orphans on boot.
///
/// # Arguments
/// * `log` - Logger to which to log shit
/// * `tx` - this is the websocket channel to which we can send websocket events to which get
/// dispatched to clients.
#[instrument(skip_all)]
pub async fn run_scanners(tx: EventTx) {
    if let Ok(conn) = dim_database::get_conn_logged().await {
        if let Ok(mut db_tx) = conn.read().begin().await {
            let mut libs = dim_database::library::Library::get_all(&mut db_tx).await;

            for lib in libs.drain(..) {
                info!("Starting scanner for {} with id: {}", lib.name, lib.id);

                let library_id = lib.id;
                let tx_clone = tx.clone();
                let media_type = lib.media_type;

                let provider = TMDBMetadataProvider::new("38c372f5bc572c8aadde7a802638534e");

                let provider = match media_type {
                    MediaType::Movie => Arc::new(provider.movies()) as Arc<_>,
                    MediaType::Tv => Arc::new(provider.tv_shows()) as Arc<_>,
                    _ => unreachable!(),
                };

                let mut watcher = scanner::daemon::FsWatcher::new(
                    conn.clone(),
                    library_id,
                    media_type,
                    tx_clone.clone(),
                    Arc::clone(&provider),
                );

                let conn_clone = conn.clone();

                tokio::spawn(async move {
                    let mut conn = conn_clone;
                    scanner::start(&mut conn, library_id, tx_clone.clone(), provider).await
                });

                tokio::spawn(async move {
                    watcher
                        .start_daemon()
                        .await
                        .expect("Something went wrong with the fs-watcher");
                });
            }
        }
    }
}

// #[instrument(skip(stream_manager, event_tx, rt, event_rx))]
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

    let event_repeater = websocket::event_repeater(
        tokio_stream::wrappers::UnboundedReceiverStream::new(event_rx),
        1024,
    );

    let socket_tx = event_repeater.sender();

    tokio::spawn(event_repeater.into_future());

    #[derive(Debug, Clone)]
    struct AppState {
        conn: DbConnection,
        socket_tx: websocket::EventSocketTx,
    }

    async fn ws_handler(
        ws: dim_web::axum::extract::WebSocketUpgrade,
        ConnectInfo(remote_address): ConnectInfo<SocketAddr>,
        State(AppState { conn, socket_tx }): State<AppState>,
    ) -> dim_web::axum::response::Response {
        ws.on_upgrade(move |websocket| async move {
            let (ws_tx, ws_rx) = websocket.split();

            websocket::handle_websocket_session(
                ws_tx.sink_err_into::<websocket::WsMessageError>(),
                ws_rx.filter_map(|m| async move { m.ok() }),
                Some(remote_address),
                conn,
                socket_tx,
            )
            .await;
        })
    }

    let router = dim_web::axum::Router::new()
        // .route_service("/api/v1/auth/login", warp!(auth::filters::login))
        .route(
            "/api/v1/auth/login",
            dim_web::axum::routing::post(dim_web::routes::auth::login).with_state(conn.clone()),
        )
        // .route_service("/api/v1/auth/register", warp!(auth::filters::register))
        .route(
            "/api/v1/auth/register",
            dim_web::axum::routing::post(dim_web::routes::auth::register).with_state(conn.clone()),
        )
        .route_service("/api/v1/auth/whoami", warp!(user::filters::whoami))
        .route(
            "/api/v1/auth/admin_exists",
            dim_web::axum::routing::get(dim_web::routes::auth::admin_exists)
                .with_state(conn.clone()),
        )
        .route_service(
            "/api/v1/library/*path",
            warp::service({
                routes::library::filters::library_get_self(conn.clone())
                    .or(routes::library::filters::get_all_unmatched_media(
                        conn.clone(),
                    ))
                    .or(routes::library::filters::library_delete(conn.clone()))
                    .or(routes::library::filters::library_get_self(conn.clone()))
                    .or(routes::library::filters::get_all_of_library(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/library",
            warp::service(library::filters::library_get(conn.clone()).or(
                routes::library::filters::library_post(conn.clone(), event_tx.clone()),
            )),
        )
        .route_service("/api/v1/dashboard", warp!(dashboard::filters::dashboard))
        .route_service(
            "/api/v1/dashboard/banner",
            warp!(dashboard::filters::banners),
        )
        .route_service("/api/v1/search", warp!(routes::general::filters::search))
        .route_service(
            "/api/v1/filebrowser/",
            warp!(routes::general::filters::get_directory_structure),
        )
        .route_service(
            "/api/v1/filebrowser/*path",
            warp!(routes::general::filters::get_directory_structure),
        )
        .route_service("/images/*path", warp!(statik::filters::get_image))
        .route_service(
            "/api/v1/media/*path",
            warp::service({
                routes::media::filters::get_media_by_id(conn.clone())
                    .or(routes::media::filters::get_media_files(conn.clone()))
                    .or(routes::media::filters::update_media_by_id(conn.clone()))
                    .or(routes::media::filters::delete_media_by_id(conn.clone()))
                    .or(routes::media::filters::tmdb_search(conn.clone()))
                    .or(routes::media::filters::map_progress(conn.clone()))
                    .or(routes::media::filters::get_mediafile_tree(conn.clone()))
                    .or(routes::rematch_media::filters::rematch_media_by_id(
                        conn.clone(),
                        event_tx.clone(),
                    ))
            }),
        )
        .route_service(
            "/api/v1/stream/*path",
            warp::service({
                routes::stream::filters::return_virtual_manifest(
                    conn.clone(),
                    state.clone(),
                    stream_tracking.clone(),
                )
                .or(routes::stream::filters::return_manifest(
                    conn.clone(),
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(routes::stream::filters::get_init(state.clone())
                    .recover(routes::global_filters::handle_rejection))
                .or(routes::stream::filters::should_client_hard_seek(
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(routes::stream::filters::session_get_stderr(
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(routes::stream::filters::kill_session(
                    state.clone(),
                    stream_tracking.clone(),
                ))
                .or(routes::stream::filters::get_subtitle(state.clone()))
                .or(routes::stream::filters::get_subtitle_ass(state.clone()))
                .or(routes::stream::filters::get_chunk(state.clone())
                    .recover(routes::global_filters::handle_rejection))
            }),
        )
        .route_service(
            "/api/v1/mediafile/*path",
            warp::service({
                routes::mediafile::filters::get_mediafile_info(conn.clone())
                    .or(routes::mediafile::filters::rematch_mediafile(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/tv/*path",
            warp!(routes::tv::filters::get_tv_seasons),
        )
        .route_service(
            "/api/v1/season/*path",
            warp::service({
                routes::tv::filters::patch_episode_by_id(conn.clone())
                    .or(routes::tv::filters::delete_season_by_id(conn.clone()))
                    .or(routes::tv::filters::get_season_episodes(conn.clone()))
                    .or(routes::tv::filters::patch_episode_by_id(conn.clone()))
                    .or(routes::tv::filters::delete_episode_by_id(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/episode/*path",
            warp::service({
                routes::tv::filters::patch_episode_by_id(conn.clone())
                    .or(routes::tv::filters::delete_episode_by_id(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/user/settings",
            warp::service({
                settings::filters::get_user_settings(conn.clone())
                    .or(routes::settings::filters::get_user_settings(conn.clone()))
                    .or(routes::settings::filters::post_user_settings(conn.clone()))
                    .or(routes::settings::filters::get_global_settings(conn.clone()))
                    .or(routes::settings::filters::set_global_settings(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/host/settings",
            warp::service({
                routes::settings::filters::get_global_settings(conn.clone())
                    .or(routes::settings::filters::set_global_settings(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/user/*path",
            warp::service({
                user::filters::change_password(conn.clone())
                    .or(user::filters::delete(conn.clone()))
                    .or(user::filters::change_username(conn.clone()))
                    .or(user::filters::upload_avatar(conn.clone()))
            }),
        )
        .route_service(
            "/api/v1/auth/*path",
            warp::service({
                invites::filters::get_all_invites(conn.clone())
                    .or(invites::filters::generate_invite(conn.clone()))
                    .or(invites::filters::delete_token(conn.clone()))
            }),
        )
        .route_service("/", warp::service(routes::statik::filters::react_routes()))
        .route_service(
            "/*path",
            warp::service(routes::statik::filters::react_routes()),
        )
        .route_service(
            "/static/*path",
            warp::service(routes::statik::filters::dist_static()),
        )
        .route("/ws", dim_web::axum::routing::get(ws_handler))
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

    info!("Webserver is listening on 0.0.0.0:{}", port);

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let web_fut = dim_web::serve(&socket_addr, router);

    tokio::select! {
        _ = web_fut => {},
        _ = tokio::signal::ctrl_c() => {
            std::process::exit(0);
        }
    }
}
