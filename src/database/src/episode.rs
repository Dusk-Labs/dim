use crate::media::InsertableMedia;
use crate::media::Media;
use crate::media::UpdateMedia;
use crate::movie::InsertableMovie;
use crate::schema::episode;
use crate::season::Season;
use crate::streamablemedia::StreamableMedia;
use crate::tv::TVShow;
use diesel::prelude::*;

/// Episode struct encapsulates a media entry representing a episode
#[derive(Serialize, Debug)]
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
    pub fn get_all_of_tv(
        conn: &diesel::PgConnection,
        media: &Media,
    ) -> Result<Vec<Episode>, diesel::result::Error> {
        use crate::schema::media;

        let tv_show = TVShow::belonging_to(media).first::<TVShow>(conn)?;
        Ok(Season::belonging_to(&tv_show)
            .load::<Season>(conn)?
            .iter()
            .map(|x| {
                EpisodeWrapper::belonging_to(x)
                    .load::<EpisodeWrapper>(conn)
                    .unwrap()
                    .iter()
                    .map(|l| {
                        (
                            *l,
                            media::dsl::media
                                .filter(media::dsl::id.eq(l.id))
                                .first::<Media>(conn)
                                .unwrap(),
                        )
                    })
                    .map(|(l, z)| l.into(z))
                    .collect::<Vec<Episode>>()
            })
            .flatten()
            .collect::<Vec<Episode>>())
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
    pub fn get_all_of_season(
        conn: &diesel::PgConnection,
        media: &Season,
    ) -> Result<Vec<Episode>, diesel::result::Error> {
        use crate::schema::media;

        Ok(EpisodeWrapper::belonging_to(media)
            .load::<EpisodeWrapper>(conn)
            .unwrap()
            .iter()
            .map(|l| {
                (
                    *l,
                    media::dsl::media
                        .filter(media::dsl::id.eq(l.id))
                        .first::<Media>(conn)
                        .unwrap(),
                )
            })
            .map(|(l, z)| l.into(z))
            .collect::<Vec<Episode>>())
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
    pub fn get(
        conn: &diesel::PgConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<Episode, diesel::result::Error> {
        use crate::schema::media;
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let season = Season::belonging_to(&tv_show)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let episode = EpisodeWrapper::belonging_to(&season)
            .filter(episode::dsl::episode_.eq(ep_num))
            .first::<EpisodeWrapper>(conn)?;

        let media = media::dsl::media
            .filter(media::dsl::id.eq(episode.id))
            .first::<Media>(conn)?;

        let result = episode.into(media);

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
    pub fn delete(
        conn: &diesel::PgConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::season;
        use crate::schema::tv_show;
        let tv_show = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let season = Season::belonging_to(&tv_show)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let episode = EpisodeWrapper::belonging_to(&season)
            .filter(episode::dsl::episode_.eq(ep_num))
            .first::<EpisodeWrapper>(conn)?;

        Media::delete(conn, episode.id)?;
        Ok(diesel::delete(&episode).execute(conn)?)
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
    pub fn insert(
        &self,
        conn: &diesel::PgConnection,
        id: i32,
    ) -> Result<i32, diesel::result::Error> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let _tv_show = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let season = season::table.find(self.seasonid).first::<Season>(conn)?;

        let media_id = self
            .media
            .into_streamable::<InsertableMovie>(conn, Some(()))?; // we use InsertableMovie with Some as it doesnt matter

        let episode: InsertableEpisodeWrapper = self.into();

        diesel::insert_into(episode::table)
            .values((
                episode::dsl::id.eq(media_id),
                episode,
                episode::dsl::seasonid.eq(season.id),
            ))
            .returning(episode::id)
            .get_result(conn)
    }

    fn into(&self) -> InsertableEpisodeWrapper {
        InsertableEpisodeWrapper {
            episode_: self.episode,
        }
    }
}

impl EpisodeWrapper {
    fn into(self, media: Media) -> Episode {
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
    pub fn update(
        &self,
        conn: &diesel::PgConnection,
        id: i32,
        season_num: i32,
        ep_num: i32,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::season;
        use crate::schema::tv_show;

        let tv = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;
        let season = Season::belonging_to(&tv)
            .filter(season::dsl::season_number.eq(season_num))
            .first::<Season>(conn)?;

        let episode = EpisodeWrapper::belonging_to(&season)
            .filter(episode::dsl::episode_.eq(ep_num))
            .first::<EpisodeWrapper>(conn)?;

        let _ = self.media.update(conn, episode.id);
        let _ = diesel::update(&episode).set(self.into()).execute(conn);
        Ok(())
    }

    fn into(&self) -> UpdateEpisodeWrapper {
        UpdateEpisodeWrapper {
            seasonid: self.seasonid,
            episode_: self.episode,
        }
    }
}
