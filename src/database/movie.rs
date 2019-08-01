use crate::database::media::*;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[derive(Queryable, Serialize, Deserialize)]
#[table_name = "movie"]
pub struct Movie {
    pub media: Media,
}

pub struct InsertableMovie {
    pub id: i32,
}

impl Movie {
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
            .select(
                media::all_columns,
                )
            .load(conn)?;
        Ok(Json(result))
    }
}
