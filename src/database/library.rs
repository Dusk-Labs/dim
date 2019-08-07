use crate::database::media::*;
use crate::schema::library;
use diesel::prelude::*;
use rocket_contrib::json::Json;

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
    pub fn get_all(conn: &diesel::SqliteConnection) -> Json<Vec<Self>> {
        use crate::schema::library::dsl::*;

        library
            .load::<Self>(conn)
            .map(|x| Json(x))
            .expect("Error querying all libraries")
    }

    pub fn get(
        conn: &diesel::SqliteConnection,
        lib_id: i32,
    ) -> Result<Json<Vec<Media>>, diesel::result::Error> {
        use crate::schema::library::dsl::*;
        let result = library.filter(id.eq(lib_id)).first::<Self>(conn)?;

        Media::get_all(conn, lib_id, result)
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

impl InsertableLibrary {
    pub fn new(&self, conn: &diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
        let result = diesel::insert_into(library::table)
            .values(self)
            .execute(conn)?;
        Ok(result)
    }
}
