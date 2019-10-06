use crate::library::Library;
use crate::schema::media;
use crate::streamablemedia::InsertableStreamableMedia;
use crate::streamablemedia::StreamableTrait;
use crate::tv::StaticTrait;
use diesel::prelude::*;

pub trait MediaTrait {}

#[derive(Clone, Identifiable, Queryable, Serialize, Deserialize, Debug, Associations)]
#[belongs_to(Library, foreign_key = "library_id")]
#[table_name = "media"]
pub struct Media {
    pub id: i32,
    pub library_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
    pub added: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub media_type: Option<String>,
}

impl PartialEq for Media {
    fn eq(&self, other: &Media) -> bool {
        self.id == other.id
    }
}

/// We literally never want to select `name_search_index`
/// so we provide this type and constant to pass to `.select`
type MediaAllColumns = (
    media::id,
    media::library_id,
    media::name,
    media::description,
    media::rating,
    media::year,
    media::added,
    media::poster_path,
    media::backdrop_path,
    media::media_type,
);

pub const MEDIA_ALL_COLUMNS: MediaAllColumns = (
    media::id,
    media::library_id,
    media::name,
    media::description,
    media::rating,
    media::year,
    media::added,
    media::poster_path,
    media::backdrop_path,
    media::media_type,
);

#[derive(Default, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "media"]
pub struct InsertableMedia {
    pub library_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
    pub added: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub media_type: String,
}

#[derive(Default, AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "media"]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
    pub added: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub media_type: Option<String>,
}

impl Media {
    pub fn get_all(
        conn: &diesel::PgConnection,
        _lib_id: i32,
        library: Library,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let result = Self::belonging_to(&library)
            .filter(media::media_type.ne("episode"))
            .select(MEDIA_ALL_COLUMNS)
            .load::<Self>(conn)?;
        Ok(result)
    }

    pub fn get(conn: &diesel::PgConnection, req_id: i32) -> Result<Self, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = media
            .filter(id.eq(req_id))
            .select(MEDIA_ALL_COLUMNS)
            .first::<Self>(conn)?;

        Ok(result)
    }

    pub fn get_by_name_and_lib(
        conn: &diesel::PgConnection,
        library: &Library,
        name: &str,
    ) -> Result<Self, diesel::result::Error> {
        let result = Self::belonging_to(library)
            .select(MEDIA_ALL_COLUMNS)
            .load::<Self>(conn)?;

        // Manual filter because of a bug with combining filter with belonging_to
        for i in result {
            if i.name == *name {
                return Ok(i);
            }
        }

        Err(diesel::result::Error::NotFound)
    }

    pub fn delete(
        conn: &diesel::PgConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = diesel::delete(media.filter(id.eq(id_to_del))).execute(conn)?;
        Ok(result)
    }
}

impl InsertableMedia {
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        library
            .filter(id.eq(self.library_id))
            .first::<Library>(conn)?;

        let result = diesel::insert_into(media::table)
            .values(self)
            .returning(media::id)
            .get_result(conn)?;

        Ok(result)
    }

    pub fn into_streamable<T: StreamableTrait>(
        &self,
        conn: &diesel::PgConnection,
        manual_t: Option<()>,
    ) -> Result<i32, diesel::result::Error> {
        let id = self.insert(conn).unwrap();
        let _ = InsertableStreamableMedia::insert(id, conn)?;
        match manual_t {
            Some(_) => Ok(id),
            None => T::new(id).insert(conn),
        }
    }

    pub fn into_static<T: StaticTrait>(
        &self,
        conn: &diesel::PgConnection,
    ) -> Result<i32, diesel::result::Error> {
        let id = self.insert(conn).unwrap();
        T::new(id).insert(conn)
    }
}

impl UpdateMedia {
    pub fn update(
        &self,
        conn: &diesel::PgConnection,
        _id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let entry = media.filter(id.eq(_id));

        diesel::update(entry).set(self).execute(conn)
    }
}
