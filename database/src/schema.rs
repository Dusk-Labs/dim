table! {
    episode (id) {
        id -> Int4,
        seasonid -> Int4,
        episode_ -> Int4,
    }
}

table! {
    genre (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    genre_media (id) {
        id -> Int4,
        genre_id -> Int4,
        media_id -> Int4,
    }
}

table! {
    library (id) {
        id -> Int4,
        name -> Varchar,
        location -> Varchar,
        media_type -> crate::library::MediaTypeMapping,
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
        backdrop_path -> Nullable<Text>,
        media_type -> Nullable<crate::library::MediaTypeMapping>,
    }
}

table! {
    mediafile (id) {
        id -> Int4,
        media_id -> Nullable<Int4>,
        library_id -> Int4,
        target_file -> Text,
        raw_name -> Text,
        raw_year -> Nullable<Int4>,
        quality -> Nullable<Varchar>,
        codec -> Nullable<Varchar>,
        container -> Nullable<Varchar>,
        audio -> Nullable<Varchar>,
        original_resolution -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
        episode -> Nullable<Int4>,
        season -> Nullable<Int4>,
        corrupt -> Nullable<Bool>,
    }
}

table! {
    movie (id) {
        id -> Int4,
    }
}

table! {
    progress (id) {
        id -> Int4,
        delta -> Int4,
        media_id -> Int4,
        user_id -> Text,
        populated -> Int4,
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

table! {
    users (username) {
        username -> Text,
        password -> Text,

        // NOTE: Sqlite doesnt support arrays, so we hack around and concat roles into a string
        // which we split later. This is a compromise as to not complicate the db crate any
        // further.
        roles -> Text,
        profile_picture -> Text,
        settings -> Text,
    }
}

table! {
    invites (id) {
        id -> Int4,
        token -> Text,
    }
}

joinable!(episode -> season (seasonid));
joinable!(episode -> streamable_media (id));
joinable!(genre_media -> genre (genre_id));
joinable!(genre_media -> media (media_id));
joinable!(media -> library (library_id));
joinable!(mediafile -> media (media_id));
joinable!(mediafile -> library (library_id));
joinable!(mediafile -> streamable_media (media_id));
joinable!(movie -> streamable_media (id));
joinable!(progress -> media (media_id));
joinable!(progress -> users (user_id));
joinable!(season -> tv_show (tvshowid));
joinable!(streamable_media -> media (id));
joinable!(tv_show -> media (id));

allow_tables_to_appear_in_same_query!(
    episode,
    genre,
    genre_media,
    invites,
    library,
    media,
    mediafile,
    movie,
    progress,
    season,
    streamable_media,
    tv_show,
    users,
);
