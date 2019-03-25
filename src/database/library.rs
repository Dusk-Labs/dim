use crate::database::media::*;
use crate::schema::library;
use diesel::prelude::*;
use rocket_contrib::json::Json

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
#[table_name = "library"]
pub struct Library {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub media_type: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "library"]
pub struct InsertableLibrary {
    pub name: String,
    pub location: String,
    pub media_type: String,
}

impl Library {
    pub fn get_all(conn: &diesel::SqliteConnection) -> Json<Vec<Library>> {
        use crate::schema::library::dsl::*;

        library
            .load::<Library>(conn)
            .map(|x| Json(x))
            .expect("Error querying all libraries")
    }

    pub fn get(
        conn: &diesel::SqliteConnection,
        lib_id: i32,
    ) -> Result<Json<Vec<Media>>, diesel::result::Error> {
        use crate::schema::library::dsl::*;
        let result = library.filter(id.eq(lib_id)).first::<Library>(conn)?;

        Media::get_all(conn, lib_id, result)
    }

    pub fn new(
        conn: &diesel::SqliteConnection,
        data: Json<InsertableLibrary>,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::library;

        let result = diesel::insert_into(library::table)
            .values(&*data)
            .execute(conn)?;
        Ok(result)
    }

    pub fn delete(
        conn: &diesel::SqliteConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        let result = diesel::delete(library.filter(id.eq(id_to_del))).execute(conn)?;
        Ok(result)
    }
}
