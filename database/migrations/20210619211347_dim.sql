-- Library table
CREATE TABLE library (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    media_type TEXT NOT NULL
);

-- Media table
-- This table contains the template for
-- the movie and tv shows tables minus containing
-- the paths because movies are streamable while
-- tv shows generally arent
-- The Episodes table will also inherit from here
CREATE TABLE media (
    id INTEGER NOT NULL,
    library_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    description TEXT,
    rating INTEGER,
    year INTEGER,
    added TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    media_type media_type NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX media_idx ON media(library_id, name, media_type) WHERE NOT media.media_type = "episode";
CREATE INDEX media_excl_ep_idx ON media(name) WHERE NOT media.media_type = "episode";

CREATE TABLE movie (
    id INTEGER,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES media (id) ON DELETE CASCADE
);

CREATE TABLE tv_show (
    id INTEGER,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES media (id) ON DELETE CASCADE
);

CREATE TABLE season (
    id INTEGER,
    season_number INTEGER NOT NULL,
    tvshowid INTEGER NOT NULL,
    added TEXT,
    poster TEXT,
    PRIMARY KEY (id),
    FOREIGN KEY(tvshowid) REFERENCES tv_show (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX season_idx ON season(season_number, tvshowid);

CREATE TABLE episode (
    id INTEGER,
    seasonid INTEGER NOT NULL,
    episode_ INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES media (id) ON DELETE CASCADE,
    FOREIGN KEY(seasonid) REFERENCES season (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX episode_idx ON episode(seasonid, episode_);

CREATE TABLE mediafile (
    -- FIXME: Have to specify NOT NULL explictly otherwise sqlx thinks this field is nullable
    id INTEGER NOT NULL,
    media_id INTEGER, -- Optional, populated on metadata search
    library_id INTEGER NOT NULL,
    target_file TEXT NOT NULL UNIQUE,

    raw_name TEXT NOT NULL,
    raw_year INTEGER,

    quality TEXT(255),
    codec TEXT(255),
    container TEXT(255),
    audio TEXT(255),
    original_resolution TEXT(255),
    duration INTEGER,
    
    episode INTEGER,
    season INTEGER,

    corrupt BOOLEAN,
    PRIMARY KEY (id),

    FOREIGN KEY(media_id) REFERENCES media (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(library_id) REFERENCES library(id) ON DELETE CASCADE
);

CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    prefs BLOB NOT NULL DEFAULT '{}',
    claimed_invite TEXT NOT NULL UNIQUE,
    roles TEXT[] NOT NULL DEFAULT 'User',

    FOREIGN KEY(claimed_invite) REFERENCES invites(id)
);

CREATE TABLE progress (
    id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    delta INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    populated INTEGER NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY(media_id) REFERENCES media (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(user_id) REFERENCES users(username) ON DELETE CASCADE
);

CREATE UNIQUE INDEX progress_idx ON progress(user_id, media_id);

CREATE TABLE genre (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE genre_media (
    id INTEGER PRIMARY KEY,
    genre_id INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (genre_id) REFERENCES genre(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX genre_media_idx ON genre_media(genre_id, media_id);

CREATE TABLE invites (
    id TEXT PRIMARY KEY NOT NULL UNIQUE,
    date_added INTEGER NOT NULL
);
