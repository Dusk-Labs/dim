use crate::media::*;
use crate::schema::library;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Identifiable, Clone)]
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
    pub fn get_all(conn: &diesel::PgConnection) -> Vec<Self> {
        use crate::schema::library::dsl::*;

        library
            .load::<Self>(conn)
            .expect("Error querying all libraries")
    }

    pub fn get_one(
        conn: &diesel::PgConnection,
        lib_id: i32,
    ) -> Result<Library, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        library.filter(id.eq(lib_id)).first::<Self>(conn)
    }

    pub fn get(
        conn: &diesel::PgConnection,
        lib_id: i32,
    ) -> Result<Vec<Media>, diesel::result::Error> {
        use crate::schema::library::dsl::*;
        let result = library.filter(id.eq(lib_id)).first::<Self>(conn)?;

        Media::get_all(conn, lib_id, result)
    }

    pub fn delete(
        conn: &diesel::PgConnection,
        id_to_del: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::library::dsl::*;

        let result = diesel::delete(library.filter(id.eq(id_to_del))).execute(conn)?;

        Ok(result)
    }
}

impl InsertableLibrary {
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(library::table)
            .values(self)
            .returning(library::id)
            .get_result(conn)
    }
}
