use crate::database::media::*;
use crate::schema::tv_show;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(Media, foreign_key = "id")]
#[table_name = "tv_show"]
pub struct TVShow {
    pub id: i32,
}

#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "tv_show"]
pub struct InsertableTVShow {
    pub id: i32,
}

impl TVShow {
    pub fn get(
        conn: &diesel::SqliteConnection,
        req_id: i32,
    ) -> Result<Json<Media>, diesel::result::Error> {
        use crate::schema::media::dsl::*;
        let result = media.filter(id.eq(req_id)).first(conn)?;
        println!("{:?}", result);
        Ok(Json(result))
    }

    pub fn get_all(
        conn: &diesel::SqliteConnection,
    ) -> Result<Json<Vec<Media>>, diesel::result::Error> {
        use crate::schema::media;
        use crate::schema::movie;
        let result = media::dsl::media
            .inner_join(movie::dsl::movie)
            .select(media::all_columns)
            .load(conn)?;
        Ok(Json(result))
    }
}

impl InsertableTVShow {
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
        let count = diesel::insert_into(tv_show::table)
            .values(self)
            .execute(conn)?;
        Ok(count)
    }
}
