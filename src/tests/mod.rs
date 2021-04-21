/*use lazy_static::lazy_static;
#[allow(unused_imports)]
use {
    rocket::config::{ConfigBuilder, Environment, LoggingLevel},
    rocket::local::Client,
};

use std::sync::{Arc, Mutex};
*/

/*
#[cfg(test)]
lazy_static! {
    pub static ref CLIENT: Arc<Mutex<Client>> = {
        let _ = database::get_conn_devel().unwrap(); // Force dim to apply migrations before mounting rocket
        let logger = crate::build_logger(true);
        let tokio_rt = tokio::runtime::Runtime::new().unwrap();
        let event_tx = tokio_rt.block_on(crate::core::start_event_server());
        let rocket_config = ConfigBuilder::new(Environment::Development)
            .address("0.0.0.0")
            .port(8001)
            .workers(64)
            .log_level(LoggingLevel::Off)
            .extra("databases", {
                let mut db_conf = std::collections::HashMap::new();
                let mut m = std::collections::HashMap::new();
                m.insert("url", "postgres://postgres:dimpostgres@127.0.0.1/dim_devel");
                db_conf.insert("dimpostgres", m);
                db_conf
            })
            .finalize()
            .unwrap();

        let state_manager = nightfall::StateManager::new(
            "/dev/null".into(),
            crate::streaming::FFPROBE_BIN.to_string(),
            crate::streaming::FFMPEG_BIN.to_string()
        );

        let rocket = crate::core::rocket_pad(logger, event_tx, rocket_config, state_manager);
        Arc::new(Mutex::new(Client::new(rocket).expect("Rocket client")))
    };
}
*/

/*
#[cfg(test)]
pub fn drop_all_data() {
    use database::get_conn_devel;
    use database::schema::*;
    use diesel::prelude::*;
    use diesel::sql_query;

    let conn = get_conn_devel().expect("Failed to get db");

    diesel::delete(library::table).execute(&conn).unwrap();
    diesel::delete(media::table).execute(&conn).unwrap();
    diesel::delete(tv_show::table).execute(&conn).unwrap();
    diesel::delete(season::table).execute(&conn).unwrap();
    diesel::delete(episode::table).execute(&conn).unwrap();
    diesel::delete(genre::table).execute(&conn).unwrap();
    diesel::delete(mediafile::table).execute(&conn).unwrap();

    let _ = sql_query("ALTER SEQUENCE library_id_seq RESTART WITH 1").execute(&conn);
    let _ = sql_query("ALTER SEQUENCE media_id_seq RESTART WITH 1").execute(&conn);
}

pub fn put_garbage() {
    use database::get_conn_devel;
    use database::{
        library::{InsertableLibrary, MediaType},
        media::InsertableMedia,
    };

    let conn = get_conn_devel().unwrap();

    let library_id = InsertableLibrary {
        name: "unittest".into(),
        location: "/dev/null".into(),
        media_type: MediaType::Movie,
    }
    .insert(&conn)
    .unwrap();

    let _media_id = InsertableMedia {
        library_id,
        name: "unittest".into(),
        added: "unittest".into(),
        media_type: MediaType::Movie,
        ..Default::default()
    }
    .insert(&conn)
    .unwrap();
}
*/

/*
pub mod route_library_tests;
pub mod route_media_tests;
*/
