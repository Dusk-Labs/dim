use crate::database::media::*;
use crate::schema::library;
use crate::core::spawn_scanner;
use crate::core::stop_scanner;
use crate::core::MediaType;
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

        let _ = stop_scanner(id_to_del as u32);

        Ok(result)
    }
}

impl InsertableLibrary {
    pub fn new(&self, conn: &diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
        let size = diesel::insert_into(library::table)
            .values(self)
            .execute(conn)?;

        let result = library::table
            .order(library::id.desc())
            .limit(size as i64)
            .load::<Library>(conn)?
            .into_iter()
            .rev()
            .last()
            .unwrap();

        let m_type = match result.media_type.as_str() {
            "movie" | "movies" => MediaType::Movie,
            _ => MediaType::TV,
        };

        let _ = spawn_scanner(result.id as u32, m_type, result.location);

        Ok(size)
    }
}
