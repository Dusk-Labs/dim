#[allow(unused_imports)]
use rocket::local::Client;

#[cfg(test)]
pub fn drop_all_data() {
    use diesel::prelude::*;
    use diesel::sql_query;
    use dim_database::get_conn;
    use dim_database::schema::*;

    let conn = get_conn().expect("Failed to get db");

    diesel::delete(library::table).execute(&conn).unwrap();
    diesel::delete(media::table).execute(&conn).unwrap();
    diesel::delete(tv_show::table).execute(&conn).unwrap();
    diesel::delete(season::table).execute(&conn).unwrap();
    diesel::delete(episode::table).execute(&conn).unwrap();
    diesel::delete(genre::table).execute(&conn).unwrap();
    diesel::delete(mediafile::table).execute(&conn).unwrap();

    let _ = sql_query("ALTER SEQUENCE library_id_seq RESTART WITH 1").execute(&conn);
}

pub mod route_library_tests;
pub mod route_media_tests;
