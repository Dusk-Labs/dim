table! {
    episode (id) {
        id -> Int4,
        seasonid -> Int4,
        episode_ -> Int4,
    }
}

table! {
    library (id) {
        id -> Int4,
        name -> Varchar,
        location -> Varchar,
        media_type -> Varchar,
    }
}

table! {
    media (id) {
        id -> Int4,
        library_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        rating -> Nullable<Int4>,
        year -> Nullable<Int4>,
        added -> Nullable<Text>,
        poster_path -> Nullable<Text>,
        media_type -> Nullable<Varchar>,
    }
}

table! {
    mediafile (id) {
        id -> Int4,
        media_id -> Nullable<Int4>,
        target_file -> Text,
        quality -> Nullable<Varchar>,
        codec -> Nullable<Varchar>,
        audio -> Nullable<Varchar>,
        original_resolution -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
    }
}

table! {
    movie (id) {
        id -> Int4,
    }
}

table! {
    season (id) {
        id -> Int4,
        season_number -> Int4,
        tvshowid -> Int4,
        added -> Nullable<Text>,
        poster -> Nullable<Text>,
    }
}

table! {
    streamable_media (id) {
        id -> Int4,
    }
}

table! {
    tv_show (id) {
        id -> Int4,
    }
}

joinable!(episode -> season (seasonid));
joinable!(episode -> streamable_media (id));
joinable!(media -> library (library_id));
joinable!(mediafile -> streamable_media (media_id));
joinable!(movie -> streamable_media (id));
joinable!(season -> tv_show (tvshowid));
joinable!(streamable_media -> media (id));
joinable!(tv_show -> media (id));

allow_tables_to_appear_in_same_query!(
    episode,
    library,
    media,
    mediafile,
    movie,
    season,
    streamable_media,
    tv_show,
);
