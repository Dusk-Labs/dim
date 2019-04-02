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
    ) -> Result<Json<Vec<Media>>, diesel::result::Error> {
        let result = Media::belonging_to(&library)
            .load::<Media>(conn)
            .map(|x| Json(x))?;
        Ok(result)
    }

    pub fn get(
        conn: &diesel::SqliteConnection,
        req_id: i32,
    ) -> Result<Json<Media>, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = media.filter(id.eq(req_id)).first::<Media>(conn)?;

        Ok(Json(result))
    }

    pub fn new(
        conn: &diesel::SqliteConnection,
        data: Json<InsertableMedia>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::library::dsl::*;
        use crate::schema::media;

        library
            .filter(id.eq(data.library_id))
            .first::<Library>(conn)?;

        let result = diesel::insert_into(media::table)
            .values(&*data)
            .execute(conn)?;
        Ok(result)
    }

    pub fn delete(
        conn: &diesel::SqliteConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let result = diesel::delete(media.filter(id.eq(id_to_del))).execute(conn)?;
        Ok(result)
    }

    pub fn update(
        conn: &diesel::SqliteConnection,
        id: i32,
        data: Json<UpdateMedia>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::media::dsl::*;

        let entry = media.filter(id.eq(id));

        diesel::update(entry).set(&*data).execute(conn)
    }
}
