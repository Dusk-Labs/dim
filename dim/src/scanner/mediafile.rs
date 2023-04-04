//! Module contains all the code that creates and inserts basic mediafiles into the database.

use crate::external::filename::Metadata;
use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::FFPROBE_BIN;

use async_trait::async_trait;

use dim_database::mediafile::InsertableMediaFile;
use dim_database::mediafile::MediaFile;
use dim_database::DatabaseError;
use dim_database::DbConnection;
use displaydoc::Display;

use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;

use tracing::debug_span;
use tracing::error;
use tracing::warn;
use tracing::Instrument;

use thiserror::Error;

use new_xtra::prelude::*;

/// This semaphore is necessary so that we dont create too many instances of `MediafileCreator`.
/// Having too many instances would not make sense as we can support only so many instances without
/// ending up contending over the database lock.
static SEMPAHORE: Semaphore = Semaphore::const_new(12);

pub type Result<T> = ::core::result::Result<T, Error>;
pub type SqlxError = Arc<sqlx::Error>;

#[derive(Clone, Debug, Display, Error, Serialize)]
pub enum Error {
    /// The file already exists in the database.
    FileExists,
    /// Failed to acquire a read-only database transaction: {0:?}
    FailedToAcquireReader(#[serde(skip)] SqlxError),
    /// Failed to acquire a read-write database transaction: {0:?}
    FailedToAcquireWriter(#[serde(skip)] SqlxError),
    /// File passed is non-unicode.
    NonUnicodeFile,
    /// Failed to extract media information with ffprobe: {0:?}
    FfprobeError(#[serde(skip)] Arc<std::io::Error>),
    /// Failed to write mediafile to the database: {0:?}
    InsertFailed(#[serde(skip)] DatabaseError),
    /// Failed to select written mediafile from the database: {0:?}
    SelectFailed(#[serde(skip)] DatabaseError),
    /// Failed to commit mediafiles to the database: {0:?}
    CommitFailed(#[serde(skip)] SqlxError),
    /// Failed to check if mediafile exists in the database: {0:?}
    ExistanceCheckFailed(#[serde(skip)] DatabaseError),
}

/// Struct is responsible for creating `InsertableMediaFile`'s and preparing them for insertions
/// into the database. It also handles writing these files to the database, and is responsible for
/// various optimizations such as batching.
///
/// The number of instances of this struct that can exist at any given time is restricted by a
/// semaphore. If this limit is reached, `MediafileCreator::new` will wait until some slots
/// free up. This is done as an optimization so that we don't have so many instances of this
/// that we start contending over the database lock, inducing timeouts and performance bottlenecks.
pub struct MediafileCreator {
    /// Pool of database connections.
    conn: DbConnection,
    /// Library which we will assign as the owner of the mediafiles inserted by this instance.
    library_id: i64,
    /// Represents a permit that we must own for the lifetime of the instance of `Self`. It is
    /// important that we use permits so that we can control how many instances of `Self` exist at
    /// any point in time.
    _permit: SemaphorePermit<'static>,
}

impl MediafileCreator {
    /// Create a new instance of `MediafileCreator`. A instance will be returned once a slot in our
    /// sempahore frees up.
    pub async fn new(conn: DbConnection, library_id: i64) -> Self {
        // NOTE: This will never panic because `SEMPAHORE` will never be dropped.
        let permit = SEMPAHORE
            .acquire()
            .await
            .expect("Failed to acquire a permit for `MediafileCreator`");

        Self {
            conn,
            library_id,
            _permit: permit,
        }
    }

    /// Method constructs a `InsertableMediaFile` from a path to a file and the metadata extracted
    /// from its filename. In addition to using metadata from the filename, it also spawns ffprobe
    /// to obtain stream information. This method can be called concurrently.
    ///
    /// # Return
    /// In addition to normal errors, this method can return `Error::FileExists`. In this case,
    /// this method has identified that a file with the path passed already exists in the database.
    /// This isnt a hard error and is simply meant to reduce database load. This error can just be
    /// skipped.
    ///
    /// # Database access
    /// This method creates one read-only database transaction and it should not affect the
    /// concurrency of read-write transactions.
    #[tracing::instrument(skip(self))]
    pub async fn construct_mediafile(
        &self,
        file: PathBuf,
        metadata: Metadata,
    ) -> Result<InsertableMediaFile> {
        let target_file = file.to_str().ok_or(Error::NonUnicodeFile)?.to_owned();

        // NOTE: This is intended to be a fast-path in-case we have inserted the file into the
        // database a long time ago. Downstream consumers of `InsertableMediaFile` should still
        // double-check that the file isnt in the database already.
        {
            let mut tx = self
                .conn
                .read()
                .begin()
                .await
                .map_err(|e| Error::FailedToAcquireReader(e.into()))?;

            if MediaFile::exists_by_file(&mut tx, &target_file).await {
                return Err(Error::FileExists);
            }
        }

        // FIXME: This is a huge bottleneck. FFProbe is slow. I'm guessing one of the reasons is
        // that it is not embedded but rather we access an on-disk binary, and the second reason is
        // that we are io-bound here.
        //
        // To fix this we could have a actor that populates metadata information at its own pace
        // and does its scheduling. If a user ever needs to obtain the metadata, we can request it
        // and patch it immediately. This would add a initial cost to the API call, but subsequent
        // API calls will be cheap.
        let video_metadata = match FFProbeCtx::new(&FFPROBE_BIN).get_meta(&target_file).await {
            Ok(x) => x,
            Err(error) => {
                error!(?error, "Couldn't extract media information with ffprobe");
                error!(file = &target_file, "Assuming file is corrupted.");
                Default::default()
            }
        };

        Ok(InsertableMediaFile {
            library_id: self.library_id,
            media_id: None,
            target_file,

            raw_name: metadata.name,
            raw_year: metadata.year,
            season: metadata.season,
            episode: metadata.episode,

            quality: video_metadata.get_height().map(|x| x.to_string()),
            codec: video_metadata.get_video_codec(),
            container: Some(video_metadata.get_container()),
            audio: video_metadata
                .get_primary_codec("audio")
                .map(ToOwned::to_owned),
            original_resolution: Default::default(),
            duration: video_metadata.get_duration().map(|x| x as i64),
            corrupt: Some(video_metadata.is_corrupt()),
            channels: video_metadata.get_primary_channels(),
            profile: video_metadata.get_video_profile(),
            audio_language: video_metadata
                .get_audio_lang()
                .or_else(|| video_metadata.get_video_lang())
                .as_deref()
                .and_then(crate::utils::lang_from_iso639)
                .map(ToString::to_string),
        })
    }

    /// Method will insert a batch of `InsertableMediaFile` within the context of one transaction.
    /// Before inserting a file it will attempt to look up if it is already in the database, and if
    /// so it will skip it.
    ///
    /// For better parallelism, this method can be called through the `Actor` interface.
    ///
    /// # Return
    /// Method will return a list of mediafiles that should now be in the database.
    ///
    /// # Tracing spans
    /// Method instruments several futures:
    ///
    /// * `mediafile_insert` - tracks when a insert operation is called.
    /// * `mediafile_select` - tracks the select for a mediafile that happens after a insert.
    /// * `database_commit` - tracks when a database commit happens.
    #[tracing::instrument(skip(self, batch))]
    pub async fn insert_batch<'a>(
        &mut self,
        batch: impl Iterator<Item = &'a InsertableMediaFile>,
    ) -> Result<Vec<MediaFile>> {
        let mut work_done = vec![];

        let mut lock = self.conn.writer().lock_owned().await;
        let mut tx = dim_database::write_tx(&mut lock)
            .await
            .map_err(|e| Error::FailedToAcquireWriter(e.into()))?;

        for mediafile in batch {
            if mediafile
                .exists(&mut tx)
                .await
                .map_err(Error::ExistanceCheckFailed)?
            {
                warn!(?mediafile, "Mediafile already exists in the database.");
                continue;
            }

            let id = mediafile
                .insert(&mut tx)
                .instrument(debug_span!("mediafile_insert"))
                .await
                .map_err(Error::InsertFailed)?;

            work_done.push(
                MediaFile::get_one(&mut tx, id)
                    .instrument(debug_span!("mediafile_select"))
                    .await
                    .map_err(Error::SelectFailed)?,
            );
        }

        tx.commit()
            .instrument(debug_span!("database_commit"))
            .await
            .map_err(|e| Error::CommitFailed(e.into()))?;

        Ok(work_done)
    }
}

#[async_trait]
impl Actor for MediafileCreator {
    type Stop = ();

    async fn stopped(self) {}
}

pub struct InsertBatch(pub Vec<InsertableMediaFile>);

#[async_trait]
impl Handler<InsertBatch> for MediafileCreator {
    type Return = Result<Vec<MediaFile>>;

    async fn handle(&mut self, batch: InsertBatch, _: &mut Context<Self>) -> Self::Return {
        self.insert_batch(batch.0.iter()).await
    }
}
