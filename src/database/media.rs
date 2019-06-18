use crate::database::library::Library;
use crate::schema::media;
use diesel::prelude::*;
use rocket_contrib::json::Json;

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

#[derive(Insertable, Serialize, Deserialize)]
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
        conn: &diesel::SqliteConnection,
        _lib_id: i32,
        library: Library,
    ) -> Result<Json<Vec<Self>>, diesel::result::Error> {
        let result = Self::belonging_to(&library)
            .load::<Self>(conn)
            .map(|x| Json(x))?;
        Ok(result)
    }

    pub fn get(
        conn: &diesel::SqliteConnection,
        req_id: i32,
    ) -> Result<Json<Self>, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = media.filter(id.eq(req_id)).first::<Self>(conn)?;

        Ok(Json(result))
    }


    pub fn delete(
        conn: &diesel::SqliteConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = diesel::delete(media.filter(id.eq(id_to_del))).execute(conn)?;
        Ok(result)
    }

}

impl InsertableMedia {
    pub fn new(
        &self,
        conn: &diesel::SqliteConnection,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        library
            .filter(id.eq(self.library_id))
            .first::<Library>(conn)?;

        let result = diesel::insert_into(media::table)
            .values(self)
            .execute(conn)?;
        Ok(result)
    }
}

impl UpdateMedia {
    pub fn update(
        &self,
        conn: &diesel::SqliteConnection,
        _id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let entry = media.filter(id.eq(_id));

        diesel::update(entry).set(self).execute(conn)
    }
}
