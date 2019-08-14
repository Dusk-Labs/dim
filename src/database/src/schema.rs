table! {
    episode (id) {
        id -> Integer,
        seasonid -> Integer,
        episode_ -> Integer,
    }
}

table! {
    library (id) {
        id -> Integer,
        name -> Text,
        location -> Text,
        media_type -> Text,
    }
}

table! {
    media (id) {
        id -> Integer,
        library_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        rating -> Nullable<Integer>,
        year -> Nullable<Integer>,
        added -> Nullable<Text>,
        poster_path -> Nullable<Text>,
        media_type -> Nullable<Text>,
    }
}

table! {
    mediafile (id) {
        id -> Integer,
        media_id -> Nullable<Integer>,
        target_file -> Text,
        quality -> Nullable<Text>,
        codec -> Nullable<Text>,
        audio -> Nullable<Text>,
        original_resolution -> Nullable<Text>,
        duration -> Nullable<Integer>,
    }
}

table! {
    movie (id) {
        id -> Integer,
    }
}

table! {
    season (id) {
        id -> Integer,
        season_number -> Integer,
        tvshowid -> Integer,
        added -> Nullable<Text>,
        poster -> Nullable<Text>,
    }
}

table! {
    streamable_media (id) {
        id -> Integer,
    }
}

table! {
    tv_show (id) {
        id -> Integer,
    }
}

joinable!(episode -> streamable_media (id));
joinable!(media -> library (library_id));
joinable!(mediafile -> streamable_media (media_id));
joinable!(movie -> streamable_media (id));
joinable!(season -> tv_show (tvshowid));
joinable!(streamable_media -> media (id));
joinable!(tv_show -> media (id));
joinable!(movie -> media (id));

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
