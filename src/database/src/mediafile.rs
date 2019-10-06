use crate::library::Library;
use crate::streamablemedia::StreamableMedia;
use crate::media::Media;
use crate::schema::mediafile;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Associations)]
#[belongs_to(Library, foreign_key = "library_id")]
#[belongs_to(StreamableMedia, foreign_key = "media_id")]
#[table_name = "mediafile"]
pub struct MediaFile {
    pub id: i32,
    pub media_id: Option<i32>,
    pub library_id: i32,
    pub target_file: String,

    pub raw_name: String,
    pub raw_year: Option<i32>,

    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,

    /***
     * Options specific to tv show scanner hence Option<T>
    ***/
    pub episode: Option<i32>,
    pub season: Option<i32>,
    /*** ***/

    pub corrupt: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "mediafile"]
pub struct InsertableMediaFile {
    pub media_id: Option<i32>,
    pub library_id: i32,
    pub target_file: String,

    pub raw_name: String,
    pub raw_year: Option<i32>,

    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,

    /***
     * Options specific to tv show scanner hence Option<T>
    ***/
    pub episode: Option<i32>,
    pub season: Option<i32>,
    /*** ***/

    pub corrupt: Option<bool>,
}

#[derive(Default, AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "mediafile"]
pub struct UpdateMediaFile {
    pub media_id: Option<i32>,
    pub target_file: Option<String>,
    pub raw_name: Option<String>,
    pub raw_year: Option<i32>,
    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,

    /***
     * Options specific to tv show scanner hence Option<T>
    ***/
    pub episode: Option<i32>,
    pub season: Option<i32>,
    /*** ***/

    pub corrupt: Option<bool>,
}

impl MediaFile {
    pub fn get_by_lib(
        conn: &diesel::PgConnection,
        lib: &Library,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        Self::belonging_to(lib).load::<Self>(conn)
    }

    pub fn get_of_media(
        conn: &diesel::PgConnection,
        media: &Media,
    ) -> Result<Self, diesel::result::Error> {
        let streamable_media = StreamableMedia::belonging_to(media)
            .first::<StreamableMedia>(conn)?;

        let result = Self::belonging_to(&streamable_media)
            .filter(mediafile::corrupt.eq(false))
            .first::<Self>(conn)?;

        Ok(result)
    }

    pub fn get_one(conn: &diesel::PgConnection, _id: i32) -> Result<Self, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;

        let result = mediafile.filter(id.eq(_id)).first::<Self>(conn)?;

        Ok(result)
    }

    pub fn exists_by_file(conn: &diesel::PgConnection, file: &str) -> bool {
        use crate::schema::mediafile::dsl::*;
        use diesel::dsl::exists;
        use diesel::dsl::select;
        select(exists(mediafile.filter(target_file.eq(file))))
            .get_result(conn)
            .unwrap()
    }
}

impl InsertableMediaFile {
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;
        let result: i32 = diesel::insert_into(mediafile)
            .values(self)
            .returning(id)
            .get_result(conn)?;

        Ok(result)
    }
}

impl UpdateMediaFile {
    pub fn update(
        &self,
        conn: &diesel::PgConnection,
        _id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;
        let entry = mediafile.filter(id.eq(_id));

        let q = diesel::update(entry).set(self);
        q.execute(conn)
    }
}
