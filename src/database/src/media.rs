use crate::library::Library;
use crate::schema::media;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Associations)]
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
    pub media_type: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "media"]
pub struct InsertableMedia {
    pub library_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub rating: Option<i32>,
    pub year: Option<i32>,
    pub added: String,
    pub poster_path: Option<String>,
    pub media_type: String,
}

#[derive(AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "media"]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub rating: Option<Option<i32>>,
    pub year: Option<Option<i32>>,
    pub added: Option<Option<String>>,
    pub poster_path: Option<Option<String>>,
    pub media_type: Option<Option<String>>,
}

impl Media {
    pub fn get_all(
        conn: &diesel::PgConnection,
        _lib_id: i32,
        library: Library,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let result = Self::belonging_to(&library)
            .load::<Self>(conn)?;
        Ok(result)
    }

    pub fn get(
        conn: &diesel::PgConnection,
        req_id: i32,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = media.filter(id.eq(req_id)).first::<Self>(conn)?;

        Ok(result)
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
    pub fn new(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::tv::InsertableTVShow;
        use crate::schema::library::dsl::*;

        library
            .filter(id.eq(self.library_id))
            .first::<Library>(conn)?;

        let count = diesel::insert_into(media::table)
            .values(self)
            .execute(conn)?;

        let result = media::dsl::media
            .order(media::id.desc())
            .limit(count as i64)
            .load::<Media>(conn)?
            .into_iter()
            .rev()
            .last()
            .unwrap()
            .id;

        match self.media_type.as_str() {
            "tv" => {
                InsertableTVShow { id: result }.insert(conn)?;
            }
            _ => {}
        }

        Ok(result)
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
