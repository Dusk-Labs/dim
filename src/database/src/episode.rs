use crate::media::InsertableMedia;
use crate::media::Media;
use crate::media::UpdateMedia;
use crate::movie::InsertableMovie;
use crate::schema::episode;
use crate::season::Season;
use crate::streamable_media::StreamableMedia;
use crate::tv::TVShow;
use crate::DatabaseError;

use cfg_if::cfg_if;
use diesel::prelude::*;
use tokio_diesel::*;

use futures::stream;
use futures::StreamExt;

/// Episode struct encapsulates a media entry representing a episode
#[derive(Clone, Serialize, Debug)]
pub struct Episode {
    #[serde(skip_serializing)]
    /// Unique id provided by postgres
    pub id: i32,
    /// Season id foreign_key
    pub seasonid: i32,
    /// episode number
    pub episode: i32,

    /// Regerence to a media object which represents this epsiode.
    /// We are essnetially aliasing and wrapping around Media transparently, behind the
    /// scene in the db episode inherits all fields from media.
    #[serde(flatten)]
    pub media: Media,
}

/// This struct is purely used for querying episodes which later gets converted into a Episode
/// struct
#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Copy, Clone)]
#[belongs_to(StreamableMedia, foreign_key = "id")]
#[belongs_to(Season, foreign_key = "seasonid")]
#[table_name = "episode"]
pub struct EpisodeWrapper {
    pub id: i32,
    pub seasonid: i32,
    pub episode: i32,
}

#[derive(Debug)]
pub struct InsertableEpisode {
    pub media: InsertableMedia,
    pub seasonid: i32,
    pub episode: i32,
}

#[derive(Insertable)]
#[table_name = "episode"]
pub struct InsertableEpisodeWrapper {
    pub episode_: i32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateEpisode {
    pub seasonid: Option<i32>,
    pub episode: Option<i32>,

    #[serde(flatten)]
    pub media: UpdateMedia,
}

#[derive(AsChangeset)]
#[table_name = "episode"]
pub struct UpdateEpisodeWrapper {
    pub seasonid: Option<i32>,
    pub episode_: Option<i32>,
}

impl Episode {
    pub async fn get_first_for_season(
        conn: &crate::DbConnection,
        season: &Season,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::episode::dsl::*;
        use crate::schema::media;

        let wrapper = episode
            .filter(seasonid.eq(season.id))
            .order(episode_.asc())
            .first_async::<EpisodeWrapper>(conn)
            .await?;

        let ep = media::dsl::media
            .filter(media::dsl::id.eq(wrapper.id))
            .first_async::<Media>(conn)
            .await?;

        Ok(wrapper.into_episode(ep))
    }

    /// Method returns all of the episodes belonging to a tv show.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a media object which should be a tv show.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::InsertableSeason;
    /// use database::episode::{InsertableEpisode, Episode};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let new_episode = InsertableEpisode {
    ///     media: InsertableMedia {
    ///         library_id: library_id,
    ///         name: "test_episode".into(),
    ///         added: "test".into(),
    ///         media_type: MediaType::Episode,
    ///         ..Default::default()
    ///     },
    ///     seasonid: season_id,
    ///     episode: 1,
    /// };
    ///
    /// let episode_id = new_episode.insert(&conn, show_id).unwrap();
    ///
    /// let show = Media::get(&conn, show_id).unwrap();
    /// let all_episodes = Episode::get_all_of_tv(&conn, &show).unwrap();
    ///
    /// assert!(all_episodes.len() == 1);
    ///
    /// let episode = &all_episodes[0];
    /// assert_eq!(episode.id, episode_id);
    /// assert_eq!(episode.episode, 1);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get_all_of_tv(
        conn: &crate::DbConnection,
        media: &Media,
    ) -> Result<Vec<Episode>, DatabaseError> {
        use crate::schema::media;
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .filter(tv_show::dsl::id.eq(media.id))
            .first_async::<TVShow>(conn)
            .await?;

        let seasons = season::dsl::season
            .filter(season::tvshowid.eq(tv_show.id))
            .load_async::<Season>(conn)
            .await?;

        let episodes = stream::iter(seasons)
            .filter_map(|x: Season| async move {
                episode::dsl::episode
                    .filter(episode::dsl::seasonid.eq(x.id))
                    .load_async::<EpisodeWrapper>(conn)
                    .await
                    .ok()
            })
            .collect::<Vec<Vec<EpisodeWrapper>>>()
            .await;

        let episodes = episodes
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<EpisodeWrapper>>();

        Ok(stream::iter(episodes)
            .filter_map(|x: EpisodeWrapper| async move {
                media::dsl::media
                    .filter(media::dsl::id.eq(x.id))
                    .first_async::<Media>(conn)
                    .await
                    .ok()
                    .map(|y| x.into_episode(y))
            })
            .collect::<Vec<Episode>>()
            .await)
    }

    /// Method returns all of the episodes belonging to a season.
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `media` - reference to a season object/entry.
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::{InsertableSeason, Season};
    /// use database::episode::{InsertableEpisode, Episode};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let new_episode = InsertableEpisode {
    ///     media: InsertableMedia {
    ///         library_id: library_id,
    ///         name: "test_episode".into(),
    ///         added: "test".into(),
    ///         media_type: MediaType::Episode,
    ///         ..Default::default()
    ///     },
    ///     seasonid: season_id,
    ///     episode: 1,
    /// };
    ///
    /// let episode_id = new_episode.insert(&conn, show_id).unwrap();
    ///
    /// let show = Media::get(&conn, show_id).unwrap();
    /// let season = Season::get(&conn, show_id, 1).unwrap();
    ///
    /// let episodes = Episode::get_all_of_season(&conn, &season).unwrap();
    ///
    /// let episode = &episodes[0];
    /// assert_eq!(episode.id, episode_id);
    /// assert_eq!(episode.episode, 1);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get_all_of_season(
        conn: &crate::DbConnection,
        media: &Season,
    ) -> Result<Vec<Episode>, DatabaseError> {
        use crate::schema::media;

        let wrappers = episode::dsl::episode
            .filter(episode::seasonid.eq(media.id))
            .load_async::<EpisodeWrapper>(conn)
            .await?;

        Ok(stream::iter(wrappers)
            .filter_map(|x| async move {
                media::dsl::media
                    .filter(media::dsl::id.eq(x.id))
                    .first_async::<Media>(conn)
                    .await
                    .map(|y| x.into_episode(y))
                    .ok()
            })
            .collect()
            .await)
    }

    /// Method returns a episodes discriminated by episode number, season number and tv show id
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - The id of a tv show we target
    /// * `season_num` - The season we are targetting
    /// * `ep_num` - Episode we are targetting
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::InsertableSeason;
    /// use database::episode::{InsertableEpisode, Episode};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let new_episode = InsertableEpisode {
    ///     media: InsertableMedia {
    ///         library_id: library_id,
    ///         name: "test_episode".into(),
    ///         added: "test".into(),
    ///         media_type: MediaType::Episode,
    ///         ..Default::default()
    ///     },
    ///     seasonid: season_id,
    ///     episode: 1,
    /// };
    ///
    /// let episode_id = new_episode.insert(&conn, show_id).unwrap();
    ///
    /// // Get episode with show.id, season 1 and episode 1
    /// let episode = Episode::get(&conn, show_id, 1, 1).unwrap();
    ///
    /// assert_eq!(episode.id, episode_id);
    /// assert_eq!(episode.episode, 1);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn get(
        conn: &crate::DbConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<Episode, DatabaseError> {
        use crate::schema::media;
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .find(id)
            .get_result_async::<TVShow>(conn)
            .await?;

        let season = season::dsl::season
            .filter(season::tvshowid.eq(tv_show.id))
            .filter(season::dsl::season_number.eq(season_num))
            .first_async::<Season>(conn)
            .await?;

        let episode = episode::dsl::episode
            .filter(episode::seasonid.eq(season.id))
            .filter(episode::dsl::episode_.eq(ep_num))
            .first_async::<EpisodeWrapper>(conn)
            .await?;

        let media = media::dsl::media
            .filter(media::dsl::id.eq(episode.id))
            .first_async::<Media>(conn)
            .await?;

        let result = episode.into_episode(media);

        Ok(result)
    }

    /// Method deletes a episode based on the tv show id, season number, and episode number
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `id` - The id of a tv show we target
    /// * `season_num` - The season we are targetting
    /// * `ep_num` - Episode we are targetting
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::InsertableSeason;
    /// use database::episode::{InsertableEpisode, Episode};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let new_episode = InsertableEpisode {
    ///     media: InsertableMedia {
    ///         library_id: library_id,
    ///         name: "test_episode".into(),
    ///         added: "test".into(),
    ///         media_type: MediaType::Episode,
    ///         ..Default::default()
    ///     },
    ///     seasonid: season_id,
    ///     episode: 1,
    /// };
    ///
    /// let episode_id = new_episode.insert(&conn, show_id).unwrap();
    ///
    /// let show = Media::get(&conn, show_id).unwrap();
    /// let all_episodes = Episode::get_all_of_tv(&conn, &show).unwrap();
    ///
    /// assert!(all_episodes.len() == 1);
    ///
    /// let res = Episode::delete(&conn, show_id, 1, 1).unwrap();
    /// let all_episodes = Episode::get_all_of_tv(&conn, &show).unwrap();
    /// assert!(all_episodes.len() == 0);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn delete(
        conn: &crate::DbConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .find(id)
            .get_result_async::<TVShow>(conn)
            .await?;

        let season = season::dsl::season
            .filter(season::tvshowid.eq(tv_show.id))
            .filter(season::dsl::season_number.eq(season_num))
            .first_async::<Season>(conn)
            .await?;

        let episode = Box::leak(
            box episode::dsl::episode
                .filter(episode::seasonid.eq(season.id))
                .filter(episode::dsl::episode_.eq(ep_num))
                .first_async::<EpisodeWrapper>(conn)
                .await?,
        );

        Media::delete(conn, episode.id).await?;
        Ok(diesel::delete(&*episode).execute_async(conn).await?)
    }
}

impl InsertableEpisode {
    /// Method inserts a new episode into the database
    ///
    /// # Arguments
    /// * `conn` - diesel connection reference to postgres
    /// * `show_id` - the id of the tv show we are trying to asociate this episode with
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media};
    /// use database::tv::InsertableTVShow;
    /// use database::season::InsertableSeason;
    /// use database::episode::{InsertableEpisode, Episode};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let new_episode = InsertableEpisode {
    ///     media: InsertableMedia {
    ///         library_id: library_id,
    ///         name: "test_episode".into(),
    ///         added: "test".into(),
    ///         media_type: MediaType::Episode,
    ///         ..Default::default()
    ///     },
    ///     seasonid: season_id,
    ///     episode: 1,
    /// };
    ///
    /// let show = Media::get(&conn, show_id).unwrap();
    /// let all_episodes = Episode::get_all_of_tv(&conn, &show).unwrap();
    /// assert!(all_episodes.len() == 0);
    ///
    /// let episode_id = new_episode.insert(&conn, show_id).unwrap();
    ///
    /// let show = Media::get(&conn, show_id).unwrap();
    /// let all_episodes = Episode::get_all_of_tv(&conn, &show).unwrap();
    /// assert!(all_episodes.len() == 1);
    ///
    /// let episode = &all_episodes[0];
    /// assert_eq!(episode.id, episode_id);
    /// assert_eq!(episode.episode, 1);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn insert(&self, conn: &crate::DbConnection, id: i32) -> Result<i32, DatabaseError> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let _tv_show = tv_show::dsl::tv_show
            .find(id)
            .get_result_async::<TVShow>(conn)
            .await?;

        let season = season::table
            .find(self.seasonid)
            .first_async::<Season>(conn)
            .await?;

        let media_id = self.media.insert(conn).await?;
        // we use InsertableMovie with Some as it doesnt matter
        self.media
            .into_streamable::<InsertableMovie>(conn, media_id, Some(()))
            .await?;

        let episode: InsertableEpisodeWrapper = self.into();

        let query = diesel::insert_into(episode::table).values((
            episode::dsl::id.eq(media_id),
            episode,
            episode::dsl::seasonid.eq(season.id),
        ));

        // Sqlite doesnt support get_result queries, so we have to emulate it with
        // `last_insert_row` function.
        cfg_if! {
            if #[cfg(feature = "postgres")] {
                Ok(query.returning(episode::id)
                    .get_result_async(conn).await?)
            } else {
                query.execute_async(conn).await?;
                Ok(diesel::select(crate::last_insert_rowid).get_result_async::<i32>(conn).await?)
            }
        }
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
            episode: self.episode,
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
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::library::{Library, InsertableLibrary, MediaType};
    /// use database::media::{InsertableMedia, Media, UpdateMedia};
    /// use database::tv::InsertableTVShow;
    /// use database::season::InsertableSeason;
    /// use database::episode::{InsertableEpisode, Episode, UpdateEpisode};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let library = InsertableLibrary {
    ///     name: "test".into(),
    ///     location: "/dev/null".to_string(),
    ///     media_type: MediaType::Tv,
    /// };
    ///
    /// let library_id = library.insert(&conn).unwrap();
    ///
    /// let new_show = InsertableMedia {
    ///     library_id: library_id,
    ///     name: "test".into(),
    ///     added: "test".into(),
    ///     media_type: MediaType::Tv,
    ///     ..Default::default()
    /// };
    ///
    /// let show_id = new_show.into_static::<InsertableTVShow>(&conn).unwrap();
    ///
    /// let new_season = InsertableSeason {
    ///     season_number: 1,
    ///     added: "test".into(),
    ///     poster: "test".into(),
    /// };
    ///
    /// let season_id = new_season.insert(&conn, show_id).unwrap();
    ///
    /// let new_episode = InsertableEpisode {
    ///     media: InsertableMedia {
    ///         library_id: library_id,
    ///         name: "test_episode".into(),
    ///         added: "test".into(),
    ///         media_type: MediaType::Episode,
    ///         ..Default::default()
    ///     },
    ///     seasonid: season_id,
    ///     episode: 1,
    /// };
    ///
    /// let episode_id = new_episode.insert(&conn, show_id).unwrap();
    ///
    /// let update_episode = UpdateEpisode {
    ///     episode: Some(2),
    ///     seasonid: None,
    ///     media: UpdateMedia {
    ///         ..Default::default()
    ///     },
    /// };
    /// let _ = update_episode.update(&conn, show_id, 1, 1).unwrap();
    /// let old_episode = Episode::get(&conn, show_id, 1, 1);
    ///
    /// assert!(old_episode.is_err());
    ///
    /// let episode = Episode::get(&conn, show_id, 1, 2).unwrap();
    ///
    /// assert_eq!(episode.id, episode_id);
    /// assert_eq!(episode.episode, 2);
    ///
    /// Library::delete(&conn, library_id).unwrap();
    pub async fn update(
        &self,
        conn: &crate::DbConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<(), DatabaseError> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv = tv_show::dsl::tv_show
            .find(id)
            .get_result_async::<TVShow>(conn)
            .await?;

        let season = season::dsl::season
            .filter(season::dsl::tvshowid.eq(tv.id))
            .filter(season::dsl::season_number.eq(season_num))
            .first_async::<Season>(conn)
            .await?;

        let episode = Box::leak(
            box episode::dsl::episode
                .filter(episode::dsl::seasonid.eq(season.id))
                .filter(episode::dsl::episode_.eq(ep_num))
                .first_async::<EpisodeWrapper>(conn)
                .await?,
        );

        let _ = self.media.update(conn, episode.id).await;
        let _ = diesel::update(&*episode)
            .set(self.into())
            .execute_async(conn)
            .await;
        Ok(())
    }

    fn into(&self) -> UpdateEpisodeWrapper {
        UpdateEpisodeWrapper {
            seasonid: self.seasonid,
            episode_: self.episode,
        }
    }
}
