use crate::schema::{genre, genre_media};
use diesel::prelude::*;

#[derive(Clone, Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "genre"]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Identifiable, Queryable, Debug, PartialEq)]
#[table_name = "genre_media"]
pub struct GenreMedia {
    pub id: i32,
    pub genre_id: i32,
    pub media_id: i32,
}

#[derive(Insertable)]
#[table_name = "genre"]
pub struct InsertableGenre {
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "genre_media"]
pub struct InsertableGenreMedia {
    pub genre_id: i32,
    pub media_id: i32,
}

impl Genre {
    pub fn get_by_name(
        conn: &diesel::PgConnection,
        query: String,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::genre::dsl::*;
        genre.filter(name.ilike(query)).first::<Self>(conn)
    }

    pub fn get_by_media(
        conn: &diesel::PgConnection,
        query: i32,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        genre::table
            .inner_join(genre_media::table)
            .filter(genre_media::media_id.eq(query))
            .select((genre::dsl::id, genre::dsl::name))
            .load::<Self>(&*conn)
    }
}

impl InsertableGenre {
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::genre::dsl::*;

        // first check if exists
        if let Ok(x) = Genre::get_by_name(&conn, self.name.clone()) {
            return Ok(x.id);
        }

        let result = diesel::insert_into(genre)
            .values(self)
            .returning(id)
            .get_result(conn)?;

        Ok(result)
    }
}

impl InsertableGenreMedia {
    pub fn insert(&self, conn: &diesel::PgConnection) {
        use crate::schema::genre_media::dsl::*;
        let _ = diesel::insert_into(genre_media).values(self).execute(conn);
    }
}
