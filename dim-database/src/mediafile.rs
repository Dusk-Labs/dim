use crate::media::Media;
use crate::query_ext::QueryExt;
use crate::DatabaseError;

use itertools::intersperse;
use serde::Deserialize;
use serde::Serialize;
use std::iter::repeat;

/// MediaFile struct which represents a media file on the filesystem. This struct holds some basic
/// information which the video player on the front end might require.
#[derive(Serialize, PartialEq, Debug, Clone, sqlx::FromRow)]
pub struct MediaFile {
    /// Unique identifier of a mediafile.
    pub id: i64,
    /// Foreign key linking this entry to the media table or [`Media`] struct
    pub media_id: Option<i64>,
    /// Library foreign key linking this entry to the library table or [`Library`](dim-database::library::Library) struct
    pub library_id: i64,
    /// String representing the file path of the file we target. This should be a real path on the
    /// filesystem.
    pub target_file: String,

    /// Raw name that we extract from the filename using regex and the parse-torrent-name library
    pub raw_name: String,
    /// Raw year we might be able to extract from the filename using regex and the
    /// parse-torrent-name library
    pub raw_year: Option<i64>,

    /// Quality string that we might get from ffprobe when running it against our file
    pub quality: Option<String>,
    /// Codec that we might get from ffprobe when running it against our file
    pub codec: Option<String>,
    /// Container descriptor that we might get from ffprobe
    pub container: Option<String>,
    /// Audio codec specifier that we might get from ffprobe
    pub audio: Option<String>,
    /// Video resolution that we can obtain from ffprobe
    pub original_resolution: Option<String>,
    /// Duration of the video file that we obtain from ffprobe
    pub duration: Option<i64>,

    /// Episode number that we might get from using regex and the parse-torrent-name crate. This is
    /// specific to tv shows only.
    pub episode: Option<i64>,
    /// Season number that we might get from using regexa and the parse-torrent-name crate. This is
    /// specific to tv shows only.
    pub season: Option<i64>,

    /// Flag which tells us if the file is corrupted or not. ie if ffprobe cant open the file and
    /// reports no metadata this flag will be set.
    pub corrupt: Option<bool>,

    /// Audio channels count
    pub channels: Option<i64>,
    /// avc_profile
    pub profile: Option<String>,
    /// Primary audio language
    pub audio_language: Option<String>,
}

impl MediaFile {
    /// Method returns all mediafiles associated with a library.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `lib` - reference to a Library object that we will match against
    pub async fn get_by_lib(
        conn: &mut crate::Transaction<'_>,
        library_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            MediaFile,
            "SELECT * FROM mediafile WHERE library_id = ?",
            library_id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    /// Method returns all mediafiles associated with a library and filters for those not
    /// associated with a media
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `lib` - reference to a Library object that we will match against
    pub async fn get_by_lib_null_media(
        conn: &mut crate::Transaction<'_>,
        library_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            MediaFile,
            "SELECT * FROM mediafile WHERE library_id = ? AND media_id IS NULL",
            library_id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    /// Method returns all mediafiles associated with a Media object.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `lib` - reference to a Library object that we will match against
    pub async fn get_of_media(
        conn: &mut crate::Transaction<'_>,
        media_id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query_as!(
            MediaFile,
            "SELECT mediafile.* FROM mediafile
                INNER JOIN media ON media.id = mediafile.media_id
                WHERE media.id = ?",
            media_id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    pub async fn get_of_show(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<Vec<Self>, DatabaseError> {
        // FIXME: I think we can make this query a bit more efficient by adding an index on
        // mediafile.mediaid, but given how often this is called, this might not be worth it.
        Ok(sqlx::query_as!(
            MediaFile,
            "SELECT mediafile.* FROM _tblseason
                INNER JOIN episode ON _tblseason.id = episode.seasonid
                INNER JOIN mediafile ON mediafile.media_id = episode.id
                WHERE _tblseason.tvshowid = ?
                GROUP BY mediafile.id",
            id
        )
        .fetch_all(&mut **conn)
        .await?)
    }

    /// Method returns all metadata of a mediafile based on the id supplied.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `_id` - id of the mediafile object we are targetting
    pub async fn get_one(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<Self, DatabaseError> {
        Ok(
            sqlx::query_as!(MediaFile, "SELECT * FROM mediafile WHERE id = ?", id)
                .fetch_one(&mut **conn)
                .await?,
        )
    }

    /// Method returns all metadata for a set of mediafile ids.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `ids` - list of mediafile ids
    pub async fn get_many(
        conn: &mut crate::Transaction<'_>,
        ids: &[i64],
    ) -> Result<Vec<Self>, DatabaseError> {
        let placeholders = intersperse(repeat("?").take(ids.len()), ",").collect::<String>();
        let query = format!("SELECT * FROM mediafile WHERE id IN ({placeholders})");

        Ok(sqlx::query_as::<_, MediaFile>(&query)
            .bind_all(ids)
            .fetch_all(&mut **conn)
            .await?)
    }

    /// Method checks whether a mediafile entry with the filepath supplied exists or not, returning
    /// a bool.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `file` - string slice containing our filepath
    pub async fn exists_by_file(conn: &mut crate::Transaction<'_>, file: &str) -> bool {
        sqlx::query!("SELECT id FROM mediafile WHERE target_file = ?", file)
            .fetch_one(&mut **conn)
            .await
            .is_ok()
    }

    pub async fn get_by_file(
        conn: &mut crate::Transaction<'_>,
        file: &str,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query_as!(
            MediaFile,
            r#"SELECT * FROM mediafile WHERE target_file = ?"#,
            file
        )
        .fetch_one(&mut **conn)
        .await?)
    }

    /// Function will return the largest duration for a media.
    pub async fn get_largest_duration(
        conn: &mut crate::Transaction<'_>,
        media_id: i64,
    ) -> Result<i64, DatabaseError> {
        Ok(sqlx::query!(
            r#"SELECT MAX(COALESCE(mediafile.duration, 0)) as "duration!: i64" FROM mediafile
            WHERE mediafile.media_id = ?
            GROUP BY mediafile.media_id
            LIMIT 1"#,
            media_id
        )
        .fetch_one(&mut **conn)
        .await?
        .duration)
    }

    /// Method deletes mediafile matching the id supplied
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `_id` - id of the mediafile entry we want to delete
    pub async fn delete(
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM mediafile WHERE id = ?", id)
            .execute(&mut **conn)
            .await?
            .rows_affected() as usize)
    }

    /// Function deletes all mediafiles with `library_id` of lib_id. This function is used when
    /// deleting a library with a sqlite backend.
    pub async fn delete_by_lib_id(
        conn: &mut crate::Transaction<'_>,
        lib_id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(
            sqlx::query!("DELETE FROM mediafile WHERE library_id = ?", lib_id)
                .execute(&mut **conn)
                .await?
                .rows_affected() as usize,
        )
    }
}

/// Same as [`MediaFile`] except its missing the id field.
#[derive(Clone, Serialize, Debug, Default)]
pub struct InsertableMediaFile {
    pub media_id: Option<i64>,
    pub library_id: i64,
    pub target_file: String,

    pub raw_name: String,
    pub raw_year: Option<i64>,

    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i64>,

    pub channels: Option<i64>,
    pub profile: Option<String>,
    pub audio_language: Option<String>,

    /***
     * Options specific to tv show scanner hence Option<T>
     ***/
    pub episode: Option<i64>,
    pub season: Option<i64>,
    /*** ***/
    pub corrupt: Option<bool>,
}

impl InsertableMediaFile {
    /// Method checks if a mediafile with the the target file exists in the database.
    pub async fn exists(&self, conn: &mut crate::Transaction<'_>) -> Result<bool, DatabaseError> {
        let result: Option<i64> =
            sqlx::query_scalar("SELECT 1 FROM mediafile WHERE target_file = ?")
                .bind(&self.target_file)
                .fetch_optional(&mut **conn)
                .await?;

        Ok(result == Some(1))
    }

    /// Method inserts a new mediafile into the database.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    pub async fn insert(&self, conn: &mut crate::Transaction<'_>) -> Result<i64, DatabaseError> {
        let id = sqlx::query!(
            r#"
            INSERT INTO mediafile (media_id, library_id, target_file, raw_name, raw_year, quality,
            codec, container, audio, original_resolution, duration, episode, season, corrupt, channels, profile, audio_language)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        "#,
            self.media_id,
            self.library_id,
            self.target_file,
            self.raw_name,
            self.raw_year,
            self.quality,
            self.codec,
            self.container,
            self.audio,
            self.original_resolution,
            self.duration,
            self.episode,
            self.season,
            self.corrupt,
            self.channels,
            self.profile,
            self.audio_language
        )
        .execute(&mut **conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}

/// Same as [`MediaFile`] except its missing the id and library_id fields. Everything is
/// optional too.
#[derive(Clone, Default, Deserialize, PartialEq, Debug)]
pub struct UpdateMediaFile {
    pub media_id: Option<i64>,
    pub target_file: Option<String>,
    pub raw_name: Option<String>,
    pub raw_year: Option<i64>,
    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i64>,
    pub channels: Option<i64>,
    pub profile: Option<String>,
    pub audio_language: Option<String>,

    /***
     * Options specific to tv show scanner hence Option<T>
     ***/
    pub episode: Option<i64>,
    pub season: Option<i64>,
    /*** ***/
    pub corrupt: Option<bool>,
}

impl UpdateMediaFile {
    /// Method updates the columns of a mediafile entry with what is supplied. The row is selected
    /// based on its id.
    ///
    /// # Arguments
    /// * `conn` - mutable reference to a sqlx transaction.
    /// * `_id` - id of the mediafile row we are targetting
    pub async fn update(
        &self,
        conn: &mut crate::Transaction<'_>,
        id: i64,
    ) -> Result<usize, DatabaseError> {
        crate::opt_update!(conn,
            "UPDATE mediafile SET media_id = ? WHERE id = ?" => (self.media_id, id),
            "UPDATE mediafile SET target_file = ? WHERE id = ?" => (self.target_file, id),
            "UPDATE mediafile SET raw_name = ? WHERE id = ?" => (self.raw_name, id),
            "UPDATE mediafile SET raw_year = ? WHERE id = ?" => (self.raw_year, id),
            "UPDATE mediafile SET quality = ? WHERE id = ?" => (self.quality, id),
            "UPDATE mediafile SET codec = ? WHERE id = ?" => (self.codec, id),
            "UPDATE mediafile SET container = ? WHERE id = ?" => (self.container, id),
            "UPDATE mediafile SET audio = ? WHERE id = ?" => (self.audio, id),
            "UPDATE mediafile SET original_resolution = ? WHERE id = ?" => (self.original_resolution, id),
            "UPDATE mediafile SET duration = ? WHERE id = ?" => (self.duration, id),
            "UPDATE mediafile SET episode = ? WHERE id = ?" => (self.episode, id),
            "UPDATE mediafile SET season = ? WHERE id = ?" => (self.season, id),
            "UPDATE mediafile SET corrupt = ? WHERE id = ?" => (self.corrupt, id),
            "UPDATE mediafile SET channels = ? WHERE id = ?" => (self.channels, id),
            "UPDATE mediafile SET profile = ? WHERE id = ?" => (self.profile, id),
            "UPDATE mediafile SET audio_language = ? WHERE id = ?" => (self.audio_language, id)
        );

        Ok(1)
    }
}

impl Into<Media> for MediaFile {
    fn into(self) -> Media {
        Media {
            id: self.id,
            library_id: self.library_id,
            name: self.raw_name,
            ..Default::default()
        }
    }
}
