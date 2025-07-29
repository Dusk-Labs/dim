use crate::core::*;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use tracing::{debug, error, instrument};

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::copy;
use std::io::Cursor;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

const PARTITIONS: usize = 5;
static SENDER_PARTITIONS: OnceCell<[UnboundedSender<(String, String)>; PARTITIONS]> =
    OnceCell::new();

#[instrument]
pub async fn insert_into_queue(poster: String, outfile: String, immediate: bool) {
    let partition = if immediate {
        PARTITIONS - 1
    } else {
        let mut hasher = DefaultHasher::new();
        poster.as_str().hash(&mut hasher);

        (hasher.finish() % (PARTITIONS as u64 - 1)) as usize
    };

    let partitions = SENDER_PARTITIONS.get_or_init(|| {
        [(); PARTITIONS].map(|_| {
            let (tx, rx) = unbounded_channel();
            tokio::spawn(process_queue(rx));

            tx
        })
    });

    partitions[partition]
        .send((poster.clone(), outfile))
        .expect("Failed to send poster request");
}

#[instrument(skip_all)]
async fn process_queue(mut rx: UnboundedReceiver<(String, String)>) {
    while let Some((url, outfile)) = rx.recv().await {
        debug!("Trying to cache {}", url);

        match reqwest::get(url.as_str()).await {
            Ok(resp) => {
                let meta_path = METADATA_PATH.get().unwrap();
                let mut out_path = PathBuf::from(meta_path);
                out_path.push(outfile);

                debug!("Caching {} -> {:?}", url, out_path);

                let mut file = match File::create(out_path) {
                    Ok(x) => x,
                    Err(e) => {
                        error!(error = ?e, url = &url, "Failed to create local file.");
                        continue;
                    }
                };

                let bytes = match resp.bytes().await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!(error = ?e, url = &url, "Failed to acquire handle on bytes from stream.");
                        continue;
                    }
                };

                let mut content = Cursor::new(bytes);
                if let Err(e) = copy(&mut content, &mut file) {
                    error!(error = ?e, url = &url, "Failed to copy bytes into file.");
                }
            }
            Err(e) => {
                error!(e = ?e, "Failed to cache URL locally: {}", url);
            }
        }
    }
}
