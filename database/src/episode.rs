use std::convert::TryInto;

use crate::media::InsertableMedia;
use crate::media::Media;
use crate::media::UpdateMedia;
use crate::season::Season;
use crate::tv::TVShow;
use crate::DatabaseError;

use serde::{Deserialize, Serialize};

use futures::stream;
use futures::StreamExt;

/// Episode struct encapsulates a media entry representing a episode
#[derive(Clone, Serialize, Debug)]
pub struct Episode {
    #[serde(skip_serializing)]
    /// Unique id provided by postgres
    pub id: i64,
    /// Season id foreign_key
    pub seasonid: i64,
    /// episode number
    pub episode: i64,

    /// Regerence to a media object which represents this epsiode.
    /// We are essnetially aliasing and wrapping around Media transparently, behind the
    /// scene in the db episode inherits all fields from media.
    #[serde(flatten)]
    pub media: Media,
}

/// This struct is purely used for querying episodes which later gets converted into a Episode
/// struct
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct EpisodeWrapper {
    pub id: i64,
    pub seasonid: i64,
    pub episode_: i64,
}

#[derive(Debug)]
pub struct InsertableEpisode {
    pub media: InsertableMedia,
    pub seasonid: i64,
    pub episode: i64,
}

pub struct InsertableEpisodeWrapper {
    pub episode_: i64,
}

#[derive(Deserialize, Debug)]
pub struct UpdateEpisode {
    pub seasonid: Option<i64>,
    pub episode: Option<i64>,

    #[serde(flatten)]
    pub media: UpdateMedia,
}

pub struct UpdateEpisodeWrapper {
    pub seasonid: Option<i64>,
    pub episode_: Option<i64>,
}

impl Episode {
    pub async fn get_first_for_season(
        conn: &crate::DbConnection,
        season: &Season,
    ) -> Result<Self, DatabaseError> {
        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT id , seasonid , episode_  FROM episode WHERE seasonid = ? ORDER BY episode_ ASC"#,
            season.id
        )
        .fetch_one(conn)
        .await?;

        let ep = Media::get(conn, wrapper.id).await?;

        Ok(wrapper.into_episode(ep))
    }

    /// Method returns all of the episodes belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a media object which should be a tv show.
    ///
    pub async fn get_all_of_tv(
        conn: &crate::DbConnection,
        media: &Media,
    ) -> Result<Vec<Episode>, DatabaseError> {
        let _ = sqlx::query!("SELECT * FROM tv_show WHERE id = ?", media.id)
            .fetch_one(conn)
            .await?;

        let mut episodes = vec![];

        for season in Season::get_all(conn, media.id).await? {
            let wrappers = sqlx::query_as!(
                EpisodeWrapper,
                r#"SELECT id , episode_ , seasonid   FROM episode WHERE seasonid = ?"#,
                season.id
            )
            .fetch_all(conn)
            .await?;

            for wrapper in wrappers {
                if let Ok(episode) = Media::get(conn, wrapper.id as i64).await {
                    episodes.push(wrapper.into_episode(episode))
                }
            }
        }

        Ok(episodes)
    }

    /// Method returns all of the episodes belonging to a season.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a season object/entry.
    ///
    pub async fn get_all_of_season(
        conn: &crate::DbConnection,
        media: &Season,
    ) -> Result<Vec<Episode>, DatabaseError> {
        let wrappers = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT id , episode_ , seasonid   FROM episode WHERE seasonid = ?"#,
            media.id
        )
        .fetch_all(conn)
        .await?;

        let mut episodes = vec![];

        for wrapper in wrappers {
            if let Ok(episode) = Media::get(conn, wrapper.id as i64).await {
                episodes.push(wrapper.into_episode(episode))
            }
        }

        Ok(episodes)
    }

    /// Method returns a episodes discriminated by episode number, season number and tv show id
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - The id of a tv show we target
    /// * `season_num` - The season we are targetting
    /// * `ep_num` - Episode we are targetting
    ///
    pub async fn get(
        conn: &crate::DbConnection,
        tv_id: i64,
        season_num: i64,
        ep_num: i64,
    ) -> Result<Episode, DatabaseError> {
        let _ = sqlx::query!("SELECT * FROM tv_show WHERE id = ?", tv_id)
            .fetch_one(conn)
            .await?;

        let season = Season::get(conn, tv_id, season_num).await?;

        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT id , seasonid , episode_  FROM episode WHERE seasonid = ? AND episode_ = ?"#,
            season.id,
            ep_num
        )
        .fetch_one(conn)
        .await?;

        let ep = Media::get(conn, wrapper.id as i64).await?;

        Ok(wrapper.into_episode(ep))
    }

    /// Method deletes a episode based on the tv show id, season number, and episode number
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - The id of a tv show we target
    /// * `season_num` - The season we are targetting
    /// * `ep_num` - Episode we are targetting
    ///
    pub async fn delete(
        conn: &crate::DbConnection,
        tv_id: i64,
        season_num: i64,
        ep_num: i64,
    ) -> Result<usize, DatabaseError> {
        let episode = Self::get(conn, tv_id, season_num, ep_num).await?;

        Media::delete(conn, episode.id.try_into().unwrap()).await?;

        Ok(sqlx::query!("DELETE FROM episode WHERE id = ?", episode.id)
            .execute(conn)
            .await?
            .rows_affected() as usize)
    }
}

impl InsertableEpisode {
    /// Method inserts a new episode into the database
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `show_id` - the id of the tv show we are trying to asociate this episode with
    ///
    pub async fn insert(
        &self,
        conn: &crate::DbConnection,
        tv_id: i64,
        media_id: i64,
    ) -> Result<i64, DatabaseError> {
        let _ = sqlx::query!("SELECT * FROM tv_show WHERE id = ?", tv_id)
            .fetch_one(conn)
            .await?;
        let _ = Season::get(conn, tv_id, self.seasonid).await?;

        if let Some(res) = sqlx::query!(
            "SELECT id FROM episode WHERE seasonid = ? AND episode_ = ?",
            self.seasonid,
            self.episode
        )
        .fetch_optional(conn)
        .await?
        {
            return Ok(res.id);
        }

        let res = sqlx::query!(
            "INSERT INTO episode (id, seasonid, episode_) VALUES($1, $2, $3)",
            media_id,
            self.seasonid,
            self.episode
        )
        .execute(conn)
        .await?;

        Ok(res.last_insert_rowid())
    }

    fn into(&self) -> InsertableEpisodeWrapper {
        InsertableEpisodeWrapper {
            episode_: self.episode,
        }
    }
}

impl EpisodeWrapper {
    pub fn into_episode(self, media: Media) -> Episode {
        Episode {
            id: self.id,
            seasonid: self.seasonid,
            episode: self.episode_,
            media,
        }
    }
}

impl UpdateEpisode {
    /// Method updates the rows of a episode.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the tv show we'd like to discriminate against
    /// * `season_num` - the season number we want to discriminate against
    /// * `ep_num` - episode number of the entry that we want to update info of
    ///
    pub async fn update(
        &self,
        conn: &crate::DbConnection,
        id: i64,
        season_num: i64,
        ep_num: i64,
    ) -> Result<(), DatabaseError> {
        let episode = Episode::get(conn, id, season_num, ep_num).await?;

        self.media.update(conn, episode.id).await?;

        todo!();

        // let _ = diesel::update(&*episode)
        //     .set(self.into())
        //     .execute_async(conn)
        //     .await;

        Ok(())
    }

    fn into(&self) -> UpdateEpisodeWrapper {
        UpdateEpisodeWrapper {
            seasonid: self.seasonid,
            episode_: self.episode,
        }
    }
}
