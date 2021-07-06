use crate::media::InsertableMedia;
use crate::media::Media;
use crate::media::UpdateMedia;
use crate::DatabaseError;

use serde::{Deserialize, Serialize};

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

impl Episode {
    pub async fn get_first_for_season(
        conn: &crate::DbConnection,
        season_id: i64,
    ) -> Result<Self, DatabaseError> {
        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT id as "id!", seasonid, episode_
            FROM episode
            WHERE seasonid = ?
            ORDER BY episode_ ASC"#,
            season_id
        )
        .fetch_one(conn)
        .await?;

        let ep = Media::get(conn, wrapper.id).await?;

        Ok(wrapper.into_episode(ep))
    }

    pub async fn get_first_for_show(
        conn: &crate::DbConnection,
        tv_id: i64,
    ) -> Result<Self, DatabaseError> {
        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.id as "id!", seasonid, episode_
            FROM episode
            INNER JOIN season on season.id = episode.seasonid
            WHERE season.tvshowid = ?
            ORDER BY episode_ ASC, season.season_number ASC
            LIMIT 1"#,
            tv_id
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
    pub async fn get_all_of_tv(
        conn: &crate::DbConnection,
        tv_show_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        let mut episodes = vec![];

        let wrappers = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.id as "id!", episode.episode_, episode.seasonid FROM episode
                INNER JOIN season ON season.id = episode.seasonid
                INNER JOIN tv_show ON tv_show.id = season.tvshowid
                WHERE tv_show.id = ?
                ORDER BY season.season_number, episode.episode_"#,
            tv_show_id
        )
        .fetch_all(conn)
        .await?;

        for wrapper in wrappers {
            if let Ok(episode) = Media::get(conn, wrapper.id as i64).await {
                episodes.push(wrapper.into_episode(episode))
            }
        }

        Ok(episodes)
    }

    // FIXME: This function might be especially heavy on the DB.
    /// Method returns all of the episodes belonging to a season.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a season object/entry.
    pub async fn get_all_of_season(
        conn: &crate::DbConnection,
        season_id: i64,
    ) -> Result<Vec<Episode>, DatabaseError> {
        let wrappers = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT id as "id!", episode_, seasonid FROM episode WHERE seasonid = ?"#,
            season_id
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
    pub async fn get(
        conn: &crate::DbConnection,
        tv_id: i64,
        season_num: i64,
        ep_num: i64,
    ) -> Result<Episode, DatabaseError> {
        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.id as "id!", episode.episode_, episode.seasonid  FROM episode
            INNER JOIN season ON season.id = episode.seasonid
            WHERE season.tvshowid = ?
            AND season.season_number = ?
            AND episode.episode_ = ?"#,
            tv_id,
            season_num,
            ep_num
        )
        .fetch_one(conn)
        .await?;

        let ep = Media::get(conn, wrapper.id as i64).await?;

        Ok(wrapper.into_episode(ep))
    }

    pub async fn get_by_id(
        conn: &crate::DbConnection,
        episode_id: i64,
    ) -> Result<Episode, DatabaseError> {
        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.*  FROM episode
            WHERE episode.id = ?"#,
            episode_id
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
    pub async fn delete(
        conn: &crate::DbConnection,
        episode_id: i64,
    ) -> Result<usize, DatabaseError> {
        // NOTE: no need to manually delete the episode entry from `episode` because of the
        // cascade delete.
        Ok(Media::delete(conn, episode_id).await?)
    }
}

#[derive(Debug)]
pub struct InsertableEpisode {
    pub media: InsertableMedia,
    pub seasonid: i64,
    pub episode: i64,
}

impl InsertableEpisode {
    /// Method inserts a new episode into the database
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    pub async fn insert(&self, conn: &crate::DbConnection) -> Result<i64, DatabaseError> {
        let tx = conn.begin().await?;

        if let Some(r) = sqlx::query!(
            r#"SELECT id as "id!" FROM episode WHERE episode.seasonid = ? AND episode.episode_ = ?"#,
            self.seasonid,
            self.episode
        )
        .fetch_optional(conn)
        .await?
        {
            return Ok(r.id);
        }

        // NOTE: use insert blind here just in case we have conflicts between episode names.
        let media_id = self.media.insert_blind(conn).await?;
        let result = sqlx::query!(
            "INSERT INTO episode (id, episode_, seasonid)
            VALUES ($1, $2, $3)",
            media_id,
            self.episode,
            self.seasonid
        )
        .execute(conn)
        .await?
        .last_insert_rowid();

        tx.commit().await?;

        Ok(result)
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

#[derive(Deserialize, Default, Debug)]
pub struct UpdateEpisode {
    pub seasonid: Option<i64>,
    pub episode: Option<i64>,

    #[serde(flatten)]
    pub media: UpdateMedia,
}

impl UpdateEpisode {
    /// Method updates the rows of a episode.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - id of the episode we wish to update.
    pub async fn update(
        &self,
        conn: &crate::DbConnection,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        let tx = conn.begin().await?;

        self.media.update(conn, id).await?;

        crate::opt_update!(conn, tx,
            "UPDATE episode SET seasonid = ? WHERE id = ?" => (self.seasonid, id),
            "UPDATE episode SET episode_ = ? WHERE id = ?" => (self.episode, id)
        );

        tx.commit().await?;

        Ok(1)
    }
}
