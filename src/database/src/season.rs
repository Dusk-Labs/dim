use crate::schema::season;
use crate::tv::TVShow;
use diesel::prelude::*;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(TVShow, foreign_key = "tvshowid")]
#[table_name = "season"]
pub struct Season {
    pub id: i32,
    pub season_number: i32,
    pub tvshowid: i32,
    pub added: Option<String>,
    pub poster: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "season"]
pub struct InsertableSeason {
    pub season_number: i32,
    pub added: String,
    pub poster: String,
}

#[derive(AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "season"]
pub struct UpdateSeason {
    pub season_number: Option<i32>,
    pub tvshowid: Option<i32>,
    pub added: Option<String>,
    pub poster: Option<String>,
}

impl Season {
    pub fn get_all(
        conn: &diesel::PgConnection,
        tv_id: i32,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::tv_show;
        let tv_show = tv_show::dsl::tv_show
            .find(tv_id)
            .get_result::<TVShow>(conn)?;

        let result = Self::belonging_to(&tv_show).load::<Self>(conn)?;

        Ok(result)
    }

    pub fn get(
        conn: &diesel::PgConnection,
        tv_id: i32,
        season_num: i32,
    ) -> Result<Season, diesel::result::Error> {
        use crate::schema::season::dsl::*;
        use crate::schema::tv_show;
        let tv_show = tv_show::dsl::tv_show
            .find(tv_id)
            .get_result::<TVShow>(conn)?;

        let result = Self::belonging_to(&tv_show)
            .filter(season_number.eq(season_num))
            .first::<Self>(conn)?;

        Ok(result)
    }

    pub fn delete(
        conn: &diesel::PgConnection,
        tv_id: i32,
        season_num: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::tv_show;

        let tv_show = tv_show::dsl::tv_show
            .find(tv_id)
            .get_result::<TVShow>(conn)?;

        let entry =
            Season::belonging_to(&tv_show).filter(season::dsl::season_number.eq(season_num));

        let result = diesel::delete(entry).execute(conn)?;
        Ok(result)
    }
}

impl InsertableSeason {
    pub fn new(&self, conn: &diesel::PgConnection, id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::tv_show;

        // We check if the tv show exists
        // if it doesnt exist the ? operator would automatically
        // return Err(diesel::result::Error)
        let _ = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        // We insert the tvshowid separately
        let _ = diesel::insert_into(season::table)
            .values((self, season::dsl::tvshowid.eq(id)))
            .execute(conn)?;
        Ok(())
    }
}

impl UpdateSeason {
    pub fn update(
        &self,
        conn: &diesel::PgConnection,
        id: i32,
        season_num: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::tv_show;

        let tv = tv_show::dsl::tv_show.find(id).get_result::<TVShow>(conn)?;

        let entry = Season::belonging_to(&tv).filter(season::dsl::season_number.eq(season_num));

        diesel::update(entry).set(self).execute(conn)
    }
}
