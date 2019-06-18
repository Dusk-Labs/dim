use crate::database::media::*;
use diesel::prelude::*;
use crate::diesel::query_dsl::InternalJoinDsl;
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
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::media::dsl::*;
        use crate::schema::movie::dsl::*;

        let result = media.inner_join(movie).load(conn)?;

        println!("{:?}", result);
        //Ok(Json(result))
        Ok(())
    }

}
