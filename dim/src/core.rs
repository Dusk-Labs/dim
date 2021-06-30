use crate::logger::RequestLogger;
use crate::routes;
use crate::scanners;
use crate::stream_tracking::StreamTracking;

use cfg_if::cfg_if;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

use slog::debug;
use slog::error;
use slog::info;
use slog::Logger;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use self::fetcher::PosterType;

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
            let media_type = lib.media_type;

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

pub mod fetcher {
    use std::{
        cmp::Ordering,
        collections::{BTreeMap, BTreeSet},
        time::Duration,
    };

    use tokio::{select, time::Interval};

    use super::*;

    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
    pub enum PosterType {
        Banner(String),
        Season(String),
        Episode(String),
    }

    impl Ord for PosterType {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (PosterType::Banner(_), PosterType::Banner(_)) => Ordering::Equal,
                (PosterType::Banner(_), PosterType::Season(_)) => Ordering::Greater,
                (PosterType::Banner(_), PosterType::Episode(_)) => Ordering::Greater,
                (PosterType::Season(_), PosterType::Banner(_)) => Ordering::Less,
                (PosterType::Season(_), PosterType::Season(_)) => Ordering::Equal,
                (PosterType::Season(_), PosterType::Episode(_)) => Ordering::Greater,
                (PosterType::Episode(_), PosterType::Banner(_)) => Ordering::Less,
                (PosterType::Episode(_), PosterType::Season(_)) => Ordering::Less,
                (PosterType::Episode(_), PosterType::Episode(_)) => Ordering::Equal,
            }
        }
    }

    impl From<PosterType> for String {
        fn from(poster: PosterType) -> Self {
            match poster {
                PosterType::Banner(st) | PosterType::Season(st) | PosterType::Episode(st) => st,
            }
        }
    }

    pub async fn tmdb_poster_fetcher(log: Logger) {
        let (tx, mut rx): (UnboundedSender<PosterType>, UnboundedReceiver<PosterType>) =
            unbounded_channel();

        let fut = async move {
            let mut processing = BTreeSet::<PosterType>::new();

            let mut timer = tokio::time::interval(Duration::from_millis(5));

            loop {
                tokio::select! {
                    _ = timer.tick() => {
                        while let Some(poster) = processing.pop_last() {
                            let url: String = poster.clone().into();

                            match reqwest::get(url.as_str()).await {
                                Ok(resp) => {
                                    if let Some(fname) = resp.url().path_segments().and_then(|segs| segs.last()) {
                                        let meta_path = METADATA_PATH.get().unwrap();
                                        let mut out_path = PathBuf::from(meta_path);
                                        out_path.push(fname);

                                        debug!(log, "Caching {} -> {:?}", url, out_path);

                                        if let Ok(mut file) = File::create(out_path) {
                                            if let Ok(bytes) = resp.bytes().await {
                                                let mut content = Cursor::new(bytes);
                                                if let Err(e) = copy(&mut content, &mut file) {
                                                    error!(log, "Failed to cache {} locally, e={:?}", url, e);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!(log, "Failed to cache {} locally, e={:?}", url, e);
                                    processing.insert(poster);
                                },
                            }
                        }
                    }

                    Some(poster) = rx.recv() => { processing.insert(poster); }
                }
            }
        };

        tokio::spawn(fut);

        METADATA_FETCHER_TX
            .set(CloneOnDeref::new(tx))
            .expect("Failed to set METADATA_FETCHER_TX");
    }
}

/// Function spins up a new Websocket server which we use to dispatch events over to clients
/// discriminated by a URI
// TODO: Handle launch failures and fallback to a new port.
// TODO: Store the port of the server in a dynamic config which can be queried by clients in case
// the port changes as we dont want this hardcoded in.
pub async fn start_event_server() -> EventTx {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(crate::websocket::serve(
        "0.0.0.0:3012",
        tokio::runtime::Handle::current(),
        rx,
    ));

    tx
}

pub async fn warp_core(
    logger: slog::Logger,
    event_tx: EventTx,
    stream_manager: StateManager,
    rt: tokio::runtime::Handle,
    port: u16,
) {
    let conn = database::get_conn()
        .await
        .expect("Failed to grab a handle to the connection pool.");
    let request_logger = RequestLogger::new(logger.clone());

    let routes = routes::auth::auth_routes(conn.clone())
        .or(routes::general::general_router(
            conn.clone(),
            logger.clone(),
            event_tx.clone(),
        ))
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
        .or(routes::stream::stream_router(
            conn.clone(),
            stream_manager,
            Default::default(),
        ))
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
