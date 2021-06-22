use std::convert::TryInto;

use crate::media::*;
use crate::DatabaseError;

use async_trait::async_trait;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

/// Trait used as a marker to mark media entries that cannot be streamed, as in not being directly
/// linked to a file on the filesystem. For example tv shows.
#[async_trait]
pub trait StaticTrait {
    /// Required method returning a instance of a object we'd like to mark as static.
    ///
    /// # Arguments
    /// * `id` - id of a media object.
    fn new(id: i32) -> Self;
    /// Required method that inserts Self into the database returning its id.
    async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError>;
}

/// Struct represents a tv show entry in the database.
/// This is mostly used as a marker to mark shows from movies, and episodes.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TVShow {
    /// id of a media object we marked as a tv show.
    pub id: i32,
}

/// Struct represents a insertable tv show entry in the database.
/// This is mostly used as a marker to mark shows from movies, and episodes.
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct InsertableTVShow {
    /// id of a media object we'd like to mark as a tv show.
    pub id: i32,
}

impl TVShow {
    /// Method returns all the tv shows in the database.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    pub async fn get_all(conn: &crate::DbConnection) -> Result<Vec<Media>, DatabaseError> {
        Ok(sqlx::query!(
            "SELECT 
                media.id, media.library_id, media.name, media.description,
                media.rating, media.year, media.added, media.poster_path, 
                media.backdrop_path FROM media INNER JOIN tv_show ON media.id = tv_show.id"
        )
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|media| Media {
            id: media.id,
            library_id: media.library_id,
            name: media.name,
            description: media.description,
            rating: media.rating,
            year: media.year,
            added: media.added,
            poster_path: media.poster_path,
            backdrop_path: media.backdrop_path,
            media_type: crate::library::MediaType::Tv,
        })
        .collect())
    }

    /// Upgrades a TV Show object into a Media object
    pub async fn upgrade(self, conn: &crate::DbConnection) -> Result<Media, DatabaseError> {
        let media = sqlx::query!(
            "SELECT 
                media.id, media.library_id, media.name, media.description,
                media.rating, media.year, media.added, media.poster_path, 
                media.backdrop_path FROM media WHERE media.id = ?",
            self.id
        )
        .fetch_one(conn)
        .await?;

        Ok(Media {
            id: media.id,
            library_id: media.library_id,
            name: media.name,
            description: media.description,
            rating: media.rating,
            year: media.year,
            added: media.added,
            poster_path: media.poster_path,
            backdrop_path: media.backdrop_path,
            media_type: crate::library::MediaType::Tv,
        })
    }
}

#[async_trait]
impl StaticTrait for InsertableTVShow {
    fn new(id: i32) -> Self {
        Self { id }
    }

    /// Method inserts a new tv show in the database.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    ///
    async fn insert(&self, conn: &crate::DbConnection) -> Result<i32, DatabaseError> {
        let res = sqlx::query!("INSERT INTO tv_show (id) VALUES ($1)", self.id)
            .execute(conn)
            .await?;

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(query.returning(tv_show::id)
                    .on_conflict_do_nothing()
                    .get_result_async(conn).await?)
            } else {
                Ok(res.last_insert_rowid().try_into().expect("can't convert rowid from i64 to i32."))
            }
        }
    }
}

impl MediaTrait for InsertableTVShow {}
