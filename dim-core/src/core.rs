use crate::scanner;

use dim_database::library::MediaType;

use dim_extern_api::tmdb::TMDBMetadataProvider;

use once_cell::sync::OnceCell;

use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

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
