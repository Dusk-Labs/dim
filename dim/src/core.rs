use crate::balanced_or_tree;
use crate::errors::DimError;
use crate::external::tmdb::TMDBMetadataProvider;
use crate::get_global_settings;
use crate::logger::RequestLogger;
use crate::routes;
use crate::routes::global_filters::get_admin_user;
use crate::routes::*;
use crate::scanner;
use crate::stream_tracking::StreamTracking;
use crate::websocket;

use database::library::MediaType;

use http::HeaderValue;
use once_cell::sync::OnceCell;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use warp::http::status::StatusCode;
use warp::reject;
use warp::Filter;

use std::sync::Arc;

pub type StateManager = nightfall::StateManager;
pub type DbConnection = database::DbConnection;
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
    if let Ok(conn) = database::get_conn_logged().await {
        if let Ok(mut db_tx) = conn.read().begin().await {
            let mut libs = database::library::Library::get_all(&mut db_tx).await;

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

#[derive(Debug)]
struct SetTokenCookie<T> {
    pub token: Option<String>,
    pub reply: T,
}

impl<T: warp::Reply> warp::Reply for SetTokenCookie<T> {
    fn into_response(self) -> warp::reply::Response {
        let mut res = self.reply.into_response();
        if let Some(token) = self.token {
            res.headers_mut().insert(
                http::header::SET_COOKIE,
                HeaderValue::from_str(&format!("token={};Path=/", token)).unwrap(),
            );
        }
        res
    }
}

#[instrument(skip(stream_manager, event_tx, rt, event_rx))]
pub async fn warp_core(
    event_tx: EventTx,
    stream_manager: StateManager,
    rt: tokio::runtime::Handle,
    port: u16,
    event_rx: UnboundedReceiver<String>,
) {
    let state = stream_manager;
    let stream_tracking = StreamTracking::default();
    let conn = database::get_conn()
        .await
        .expect("Failed to grab a handle to the connection pool.");

    let request_logger = RequestLogger::new();

    let api_routes = balanced_or_tree![
        /* NOTE: v1 REST API routes start HERE */
        /* /api/v1/auth routes*/
        auth::filters::login(conn.clone()),
        user::filters::whoami(conn.clone()),
        host::filters::admin_exists(conn.clone()),
        auth::filters::register(conn.clone()),
        invites::filters::get_all_invites(conn.clone()),
        invites::filters::generate_invite(conn.clone()),
        invites::filters::delete_token(conn.clone()),
        /* /api/v1/user routes */
        user::filters::change_password(conn.clone()),
        user::filters::delete(conn.clone()),
        user::filters::change_username(conn.clone()),
        user::filters::upload_avatar(conn.clone()),
        /* general routes */
        routes::general::filters::search(conn.clone()),
        routes::general::filters::get_directory_structure(conn.clone()),
        /* library routes */
        routes::library::filters::library_get(conn.clone()),
        routes::library::filters::library_post(conn.clone(), event_tx.clone()),
        routes::library::filters::library_delete(conn.clone()),
        routes::library::filters::library_get_self(conn.clone()),
        routes::library::filters::get_all_of_library(conn.clone()),
        routes::library::filters::get_all_unmatched_media(conn.clone()),
        /* dashboard routes */
        routes::dashboard::filters::dashboard(conn.clone(), rt.clone()),
        routes::dashboard::filters::banners(conn.clone()),
        /* media routes */
        routes::media::filters::get_media_by_id(conn.clone()),
        routes::media::filters::get_media_files(conn.clone()),
        routes::media::filters::update_media_by_id(conn.clone()),
        routes::media::filters::delete_media_by_id(conn.clone()),
        routes::media::filters::tmdb_search(conn.clone()),
        routes::media::filters::map_progress(conn.clone()),
        routes::media::filters::get_mediafile_tree(conn.clone()),
        routes::rematch_media::filters::rematch_media_by_id(conn.clone(), event_tx.clone()),
        /* tv routes */
        routes::tv::filters::get_tv_seasons(conn.clone()),
        routes::tv::filters::patch_episode_by_id(conn.clone()),
        routes::tv::filters::delete_season_by_id(conn.clone()),
        routes::tv::filters::get_season_episodes(conn.clone()),
        routes::tv::filters::patch_episode_by_id(conn.clone()),
        routes::tv::filters::delete_episode_by_id(conn.clone()),
        /* mediafile routes */
        routes::mediafile::filters::get_mediafile_info(conn.clone()),
        routes::mediafile::filters::rematch_mediafile(conn.clone()),
        /* settings routes */
        routes::settings::filters::get_user_settings(conn.clone()),
        routes::settings::filters::post_user_settings(conn.clone()),
        routes::settings::filters::get_global_settings(conn.clone()),
        routes::settings::filters::set_global_settings(conn.clone()),
        /* stream routes */
        routes::stream::filters::return_virtual_manifest(
            conn.clone(),
            state.clone(),
            stream_tracking.clone()
        ),
        routes::stream::filters::return_manifest(
            conn.clone(),
            state.clone(),
            stream_tracking.clone()
        ),
        routes::stream::filters::get_init(state.clone())
            .recover(routes::global_filters::handle_rejection),
        routes::stream::filters::should_client_hard_seek(state.clone(), stream_tracking.clone()),
        routes::stream::filters::session_get_stderr(state.clone(), stream_tracking.clone()),
        routes::stream::filters::kill_session(state.clone(), stream_tracking.clone()),
        routes::stream::filters::get_subtitle(state.clone()),
        routes::stream::filters::get_subtitle_ass(state.clone()),
        routes::stream::filters::get_chunk(state.clone())
            .recover(routes::global_filters::handle_rejection),
        warp::path!("api" / "stream" / ..)
            .and(warp::any())
            .map(|| StatusCode::NOT_FOUND),
        routes::statik::filters::get_image(conn.clone()),
    ]
    .recover(routes::global_filters::handle_rejection);

    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            let api_routes = api_routes.boxed();
        }
    }

    let routes = balanced_or_tree![
        api_routes,
        /* NOTE: This is a barrier to 404 any rest api calls that dont match till here */
        routes::global_filters::api_not_found(),
        /* websocket route */
        websocket::event_socket(tokio::runtime::Handle::current(), event_rx, conn.clone())
            .recover(routes::global_filters::handle_rejection),
        /* static routes */
        routes::statik::filters::dist_static(),
        routes::statik::filters::react_routes(),
    ]
    .map(move |reply| (reply, conn.clone()))
    .untuple_one()
    .and_then(|reply, conn: DbConnection| async move {
        if !get_global_settings().disable_auth {
            return Ok::<_, warp::Rejection>(SetTokenCookie { token: None, reply });
        }

        let mut tx = conn.read().begin().await.map_err(|_| {
            reject::custom(DimError::DatabaseError {
                description: String::from("Failed to start transaction"),
            })
        })?;

        let token;
        if let Ok(user) = get_admin_user(&mut tx).await {
            token = Some(database::user::Login::create_cookie(user.id));
        } else {
            token = None;
        }
        Ok(SetTokenCookie { token, reply })
    })
    .recover(routes::global_filters::handle_rejection)
    .with(warp::filters::log::custom(move |x| {
        request_logger.on_response(x);
    }))
    .with(warp::cors().allow_any_origin())
    .boxed();

    info!("Webserver is listening on 0.0.0.0:{}", port);

    tokio::select! {
        _ = warp::serve(routes).run(([0, 0, 0, 0], port)) => {},
        _ = tokio::signal::ctrl_c() => {
            std::process::exit(0);
        }
    }
}
