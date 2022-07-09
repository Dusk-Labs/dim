use crate::media::InsertableMedia;
use crate::media::Media;
use crate::media::UpdateMedia;
use crate::user::UserID;
use crate::DatabaseError;

use serde::{Deserialize, Serialize};

/// Episode struct encapsulates a media entry representing a episode
#[derive(Clone, Serialize, Debug)]
pub struct Episode {
    #[serde(skip_serializing)]
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
#[derive(PartialEq, Debug, Copy, Clone, sqlx::FromRow)]
pub struct EpisodeWrapper {
    pub id: i64,
    pub seasonid: i64,
    pub episode_: i64,
}

impl Episode {
    pub async fn get_first_for_season(
        conn: &mut crate::Transaction<'_>,
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
        .fetch_one(&mut *conn)
        .await?;

        let ep = Media::get(conn, wrapper.id).await?;

        Ok(wrapper.into_episode(ep))
    }

    pub async fn get_first_for_show(
        conn: &mut crate::Transaction<'_>,
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
        .fetch_one(&mut *conn)
        .await?;

        let ep = Media::get(conn, wrapper.id).await?;

        Ok(wrapper.into_episode(ep))
    }

    /// Method returns all of the episodes belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - a mutable reference to a sqlx transaction.
    /// * `media` - reference to a media object which should be a tv show.
    pub async fn get_all_of_tv(
        conn: &mut crate::Transaction<'_>,
        tv_show_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        let mut episodes = vec![];

        let wrappers = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.id as "id!", episode.episode_, episode.seasonid FROM episode
                INNER JOIN season ON season.id = episode.seasonid
                INNER JOIN _tblmedia ON _tblmedia.id = season.tvshowid
                WHERE _tblmedia.id = ?
                ORDER BY season.season_number, episode.episode_"#,
            tv_show_id
        )
        .fetch_all(&mut *conn)
        .await?;

        for wrapper in wrappers {
            if let Ok(episode) = Media::get(&mut *conn, wrapper.id as i64).await {
                episodes.push(wrapper.into_episode(episode))
            }
        }

        Ok(episodes)
    }

    // FIXME: This function might be especially heavy on the DB.
    /// Method returns all of the episodes belonging to a season.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `media` - reference to a season object/entry.
    pub async fn get_all_of_season(
        conn: &mut crate::Transaction<'_>,
        season_id: i64,
    ) -> Result<Vec<Episode>, DatabaseError> {
        let wrappers = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT id as "id!", episode_, seasonid FROM episode WHERE seasonid = ?"#,
            season_id
        )
        .fetch_all(&mut *conn)
        .await?;

        let mut episodes = vec![];

        for wrapper in wrappers {
            if let Ok(episode) = Media::get(&mut *conn, wrapper.id as i64).await {
                episodes.push(wrapper.into_episode(episode))
            }
        }

        Ok(episodes)
    }

    /// Method returns a episodes discriminated by episode number, season number and tv show id
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `id` - The id of a tv show we target
    /// * `season_num` - The season we are targetting
    /// * `ep_num` - Episode we are targetting
    pub async fn get(
        conn: &mut crate::Transaction<'_>,
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
        .fetch_one(&mut *conn)
        .await?;

        let ep = Media::get(conn, wrapper.id as i64).await?;

        Ok(wrapper.into_episode(ep))
    }

    pub async fn get_by_id(
        conn: &mut crate::Transaction<'_>,
        episode_id: i64,
    ) -> Result<Episode, DatabaseError> {
        let wrapper = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.*  FROM episode
            WHERE episode.id = ?"#,
            episode_id
        )
        .fetch_one(&mut *conn)
        .await?;

        let ep = Media::get(conn, wrapper.id as i64).await?;

        Ok(wrapper.into_episode(ep))
    }

    pub async fn get_season_episode_by_id(
        conn: &mut crate::Transaction<'_>,
        episode_id: i64,
    ) -> Result<(i64, i64), DatabaseError> {
        struct Record {
            episode: i64,
            season: i64,
        }

        let result = sqlx::query_as!(
            Record,
            "SELECT episode_ as episode, season.season_number as season FROM episode
            INNER JOIN season ON season.id = episode.seasonid
            WHERE episode.id = ?",
            episode_id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok((result.season, result.episode))
    }

    pub async fn get_season_number(
        &self,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<i64, DatabaseError> {
        let record = sqlx::query!(
            "SELECT season.season_number FROM season
            WHERE season.id = ?",
            self.seasonid
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(record.season_number)
    }

    /// Function will query for the episode after the episode passed in.
    pub async fn get_next_episode(
        &self,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<Episode, DatabaseError> {
        let season_number = self.get_season_number(&mut *conn).await?;

        let record = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.id as "id!", episode.seasonid, episode.episode_ FROM episode
            INNER JOIN season ON season.id = episode.seasonid
            WHERE season.tvshowid = (
                SELECT _tblseason.tvshowid FROM _tblseason
                WHERE _tblseason.id = ?
            ) AND ((
                episode.episode_ > ? AND
                season.season_number = ?
            ) OR season.season_number > ?)
            ORDER BY season.season_number, episode.episode_
            LIMIT 1"#,
            self.seasonid,
            self.episode,
            season_number,
            season_number
        )
        .fetch_one(&mut *conn)
        .await?;

        let ep = Media::get(conn, record.id as i64).await?;

        Ok(record.into_episode(ep))
    }

    /// Function will query for the episode after the episode passed in.
    pub async fn get_prev_episode(
        &self,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<Episode, DatabaseError> {
        let season_number = self.get_season_number(&mut *conn).await?;

        let record = sqlx::query_as!(
            EpisodeWrapper,
            r#"SELECT episode.id as "id!", episode.seasonid, episode.episode_ FROM episode
            INNER JOIN season ON season.id = episode.seasonid
            WHERE season.tvshowid = (
                SELECT _tblseason.tvshowid FROM _tblseason
                WHERE _tblseason.id = ?
            ) AND ((
                episode.episode_ < ? AND
                season.season_number = ?
            ) OR season.season_number < ?)
            ORDER BY season.season_number DESC, episode.episode_ DESC
            LIMIT 1"#,
            self.seasonid,
            self.episode,
            season_number,
            season_number
        )
        .fetch_one(&mut *conn)
        .await?;

        let ep = Media::get(conn, record.id as i64).await?;

        Ok(record.into_episode(ep))
    }

    /// Function will query the last episode that was watched for a show.
    pub async fn get_last_watched_episode(
        conn: &mut crate::Transaction<'_>,
        tvid: i64,
        uid: UserID,
    ) -> Result<Option<Episode>, DatabaseError> {
        // FIXME: We're using the query_as function instead of macro because `LEFT OUTER JOIN`
        // crashes the proc macro.
        let result = sqlx::query_as::<_, EpisodeWrapper>(
            "SELECT episode.* FROM episode
            INNER JOIN season ON season.id = episode.seasonid
            INNER JOIN progress ON progress.media_id = episode.id AND progress.user_id = ?
            WHERE season.tvshowid = ?
            ORDER BY progress.populated DESC
            LIMIT 1",
        )
        .bind(uid)
        .bind(tvid)
        .fetch_optional(&mut *conn)
        .await?;

        let result = if let Some(r) = result {
            r
        } else {
            return Ok(None);
        };

        let ep = Media::get(conn, result.id as i64).await?;
        Ok(Some(result.into_episode(ep)))
    }

    pub async fn get_seasonid(
        tx: &mut crate::Transaction<'_>,
        episodeid: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            "SELECT episode.seasonid FROM episode WHERE episode.id = ?",
            episodeid
        )
        .fetch_one(&mut *tx)
        .await?
        .seasonid)
    }

    /// Method deletes a episode based on the tv show id, season number, and episode number
    ///
    /// # Arguments
    /// * `id` - The id of a tv show we target
    /// * `season_num` - The season we are targetting
    /// * `ep_num` - Episode we are targetting
    pub async fn delete(
        conn: &mut crate::Transaction<'_>,
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
    pub async fn insert(&self, conn: &mut crate::Transaction<'_>) -> Result<i64, DatabaseError> {
        if let Some(r) = sqlx::query!(
            r#"SELECT id as "id!" FROM episode WHERE episode.seasonid = ? AND episode.episode_ = ?"#,
            self.seasonid,
            self.episode
        )
        .fetch_optional(&mut *conn)
        .await?
        {
            return Ok(r.id);
        }

        // NOTE: use insert blind here just in case we have conflicts between episode names.
        let media_id = self.media.insert_blind(&mut *conn).await?;
        let result = sqlx::query!(
            "INSERT INTO episode (id, episode_, seasonid)
            VALUES ($1, $2, $3)",
            media_id,
            self.episode,
            self.seasonid
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

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
    /// * `id` - id of the episode we wish to update.
    pub async fn update(
        &self,
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        self.media.update(&mut *conn, id).await?;

        crate::opt_update!(conn,
            "UPDATE episode SET seasonid = ? WHERE id = ?" => (self.seasonid, id),
            "UPDATE episode SET episode_ = ? WHERE id = ?" => (self.episode, id)
        );

        Ok(1)
    }
}
