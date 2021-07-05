-- Library table
CREATE TABLE library (
    id INTEGER PRIMARY KEY,
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
    id INTEGER,
    library_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    description TEXT,
    rating INTEGER,
    year INTEGER,
    added TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    media_type media_type,
    PRIMARY KEY (id),
    FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE CASCADE
);

-- Streamble Media Table
-- This table contains the template for
-- Media that has a file attached to it
-- ie it can be streamed.
-- Currently only movies and episodes inherit from this
CREATE TABLE streamable_media (
    id INTEGER,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES media (id) ON DELETE CASCADE
);

CREATE TABLE movie (
    id INTEGER,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES streamable_media (id) ON DELETE CASCADE
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

CREATE TABLE episode (
    id INTEGER,
    seasonid INTEGER NOT NULL,
    episode_ INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES streamable_media (id) ON DELETE CASCADE,
    FOREIGN KEY(seasonid) REFERENCES season (id) ON DELETE CASCADE
);

CREATE TABLE mediafile (
    id INTEGER,
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
--  For now we directly link to media instead of a intermediary, NOTE: FIXME
    FOREIGN KEY(media_id) REFERENCES streamable_media (id) ON DELETE CASCADE ON UPDATE CASCADE,
--    FOREIGN KEY(media_id) REFERENCES media (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(library_id) REFERENCES library(id) ON DELETE CASCADE
);

CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    profile_picture TEXT NOT NULL DEFAULT 'https://i.redd.it/3n1if40vxxv31.png',
    settings TEXT NOT NULL DEFAULT '{}',
    roles TEXT[] NOT NULL DEFAULT 'User'
);

CREATE TABLE progress (
    id INTEGER,
    user_id TEXT NOT NULL,
    delta INTEGER,
    media_id INTEGER,
    populated INTEGER,

    PRIMARY KEY (id),
    FOREIGN KEY(media_id) REFERENCES media (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(user_id) REFERENCES users(username) ON DELETE CASCADE
);

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

CREATE TABLE invites (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE
);
