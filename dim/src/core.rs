use crate::logger::RequestLogger;
use crate::routes;
use crate::scanners;
use crate::fetcher::PosterType;
use crate::websocket;

use once_cell::sync::OnceCell;

use slog::Logger;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use warp::Filter;

pub type StateManager = nightfall::StateManager;
pub type DbConnection = database::DbConnection;
pub type EventTx = UnboundedSender<String>;

/// Hacky type we use to implement clone on deref types.
#[derive(Clone, Debug)]
pub struct CloneOnDeref<T> {
    inner: T,
}

impl<T: Clone> CloneOnDeref<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> T {
        self.inner.clone()
    }
}

unsafe impl<T: Send> Send for CloneOnDeref<T> {}
unsafe impl<T: Clone> Sync for CloneOnDeref<T> {}

/// Path to where metadata is stored and should be fetched to.
pub static METADATA_PATH: OnceCell<String> = OnceCell::new();
// NOTE: While the sender is wrapped in a Mutex, we dont really care as wel copy the inner type at
// some point anyway.
/// Contains the tx channel over which we can send images to be cached locally.
pub static METADATA_FETCHER_TX: OnceCell<CloneOnDeref<UnboundedSender<PosterType>>> =
    OnceCell::new();

/// Function dumps a list of all libraries in the database and starts a scanner for each which
/// monitors for new files using fsnotify. It also scans all orphans on boot.
///
/// # Arguments
/// * `log` - Logger to which to log shit
/// * `tx` - this is the websocket channel to which we can send websocket events to which get
/// dispatched to clients.
pub async fn run_scanners(log: Logger, tx: EventTx) {
    if let Ok(conn) = database::get_conn_logged(&log).await {
        for lib in database::library::Library::get_all(&conn).await {
            slog::info!(log, "Starting scanner for {} with id: {}", lib.name, lib.id);
            let log_clone = log.clone();
            let library_id = lib.id;
            let tx_clone = tx.clone();

            tokio::spawn(scanners::start(library_id, log_clone.clone(), tx_clone));

            let log_clone = log.clone();
            let library_id = lib.id;
            let tx_clone = tx.clone();
            let media_type = lib.media_type;
            tokio::spawn(async move {
                let watcher = scanners::scanner_daemon::FsWatcher::new(
                    log_clone, library_id, media_type, tx_clone,
                )
                .await;

                watcher
                    .start_daemon()
                    .await
                    .expect("Something went wrong with the fs-watcher");
            });
        }
    }
}

pub async fn warp_core(
    logger: slog::Logger,
    event_tx: EventTx,
    stream_manager: StateManager,
    rt: tokio::runtime::Handle,
    port: u16,
    event_rx: UnboundedReceiver<String>,
) {
    let conn = database::get_conn()
        .await
        .expect("Failed to grab a handle to the connection pool.");

    let request_logger = RequestLogger::new(logger.clone());

    let routes = routes::auth::auth_routes(conn.clone())
        .or(routes::general::general_router(conn.clone()))
        .or(routes::library::library_routes(
            conn.clone(),
            logger.clone(),
            event_tx.clone(),
        ))
        .or(routes::dashboard::dashboard_router(
            conn.clone(),
            rt.clone(),
        ))
        .or(routes::media::media_router(conn.clone()))
        .or(routes::tv::tv_router(conn.clone()))
        .or(routes::mediafile::mediafile_router(
            conn.clone(),
            logger.clone(),
        ))
        .or(routes::settings::settings_router(conn.clone()))
        .or(routes::stream::stream_router(
            conn.clone(),
            stream_manager,
            Default::default(),
            logger.clone(),
        ))
        .or(routes::global_filters::api_not_found())
        .or(
            websocket::event_socket(tokio::runtime::Handle::current(), event_rx)
                .recover(routes::global_filters::handle_rejection),
        )
        .or(routes::statik::statik_routes())
        .with(warp::filters::log::custom(move |x| {
            request_logger.on_response(x);
        }))
        .with(warp::cors().allow_any_origin());

    tokio::select! {
        _ = warp::serve(routes).run(([0, 0, 0, 0], port)) => {},
        _ = tokio::signal::ctrl_c() => {
            std::process::exit(0);
        }
    }
}
