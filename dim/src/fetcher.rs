use crate::core::*;

use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use tracing::{debug, error, instrument};

use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

#[instrument]
pub async fn insert_into_queue(poster: String, priority: usize) {
    // FIXME: We might want to figure out a way to make this a const generic param.
    const PARTITIONS: usize = 5;
    static SENDER_PARTITIONS: OnceCell<[UnboundedSender<(String, usize)>; PARTITIONS]> =
        OnceCell::new();

    let partitions = SENDER_PARTITIONS.get_or_init(|| {
        [(); PARTITIONS].map(|_| {
            let (tx, rx) = unbounded_channel();
            tokio::spawn(process_queue(rx));

            tx
        })
    });

    partitions[priority % PARTITIONS]
        .send((poster.clone(), priority))
        .expect("Failed to send poster request");
}

#[instrument]
async fn process_queue(mut rx: UnboundedReceiver<(String, usize)>) {
    while let Some((url, priority)) = rx.recv().await {
        debug!("Trying to cache {}", url);

        match reqwest::get(url.as_str()).await {
            Ok(resp) => {
                if let Some(fname) = resp.url().path_segments().and_then(|segs| segs.last()) {
                    let meta_path = METADATA_PATH.get().unwrap();
                    let mut out_path = PathBuf::from(meta_path);
                    out_path.push(fname);

                    debug!("Caching {} -> {:?}", url, out_path);

                    if let Ok(mut file) = File::create(out_path) {
                        if let Ok(bytes) = resp.bytes().await {
                            let mut content = Cursor::new(bytes);
                            if copy(&mut content, &mut file).is_ok() {
                                continue;
                            }
                        }
                    }
                }

                error!(
                    "Failed to cache {} locally, appending back into queue",
                    &url
                );
            }
            Err(e) => {
                error!(e = ?e, "Failed to cache URL locally: {}", url);
            }
        }
    }
}
