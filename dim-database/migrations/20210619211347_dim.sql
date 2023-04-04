-- Library table
CREATE TABLE library (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    media_type TEXT NOT NULL
);

CREATE TABLE indexed_paths (
    id INTEGER PRIMARY KEY NOT NULL,
    -- must be absolute path
    location TEXT NOT NULL UNIQUE,
    library_id INTEGER NOT NULL,

    FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE CASCADE
);

-- Media table
-- This table contains the template for
-- the movie and tv shows tables minus containing
-- the paths because movies are streamable while
-- tv shows generally arent
-- The Episodes table will also inherit from here
CREATE TABLE _tblmedia (
    id INTEGER NOT NULL,
    library_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    description TEXT,
    rating REAL,
    year INTEGER,
    added TEXT,
    poster INTEGER,
    backdrop INTEGER,
    media_type TEXT NOT NULL,
    PRIMARY KEY (id),

    FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE CASCADE,
    FOREIGN KEY (poster) REFERENCES assets(id),
    FOREIGN KEY (backdrop) REFERENCES assets(id)
);

-- Nicer view of media, ie we dont have to manually query some data.
CREATE VIEW media AS
SELECT _tblmedia.*, pp.local_path as poster_path, bp.local_path as backdrop_path
FROM _tblmedia
LEFT JOIN assets pp ON _tblmedia.poster = pp.id
LEFT JOIN assets bp ON _tblmedia.backdrop = bp.id;

CREATE TRIGGER media_delete
INSTEAD OF DELETE ON media
BEGIN
    DELETE FROM _tblmedia WHERE _tblmedia.id = old.id;
END;

CREATE UNIQUE INDEX media_idx ON _tblmedia(library_id, name, media_type) WHERE NOT _tblmedia.media_type = "episode";
CREATE INDEX media_excl_ep_idx ON _tblmedia(name) WHERE NOT _tblmedia.media_type = "episode";

CREATE TABLE _tblseason (
    id INTEGER,
    season_number INTEGER NOT NULL,
    tvshowid INTEGER NOT NULL,
    added TEXT,
    poster INTEGER,
    PRIMARY KEY (id),
    
    FOREIGN KEY(poster) REFERENCES assets(id),
    FOREIGN KEY(tvshowid) REFERENCES _tblmedia (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX season_idx ON _tblseason(season_number, tvshowid);

-- Nicer view of _tblseason, ie we dont have to manually query some data.
CREATE VIEW season AS
SELECT _tblseason.id, _tblseason.season_number,
    _tblseason.tvshowid, _tblseason.added, assets.local_path as poster
FROM _tblseason
JOIN assets ON _tblseason.poster = assets.id;

CREATE TRIGGER season_delete
INSTEAD OF DELETE ON season
BEGIN
    DELETE FROM _tblseason WHERE _tblseason.id = old.id;
END;

CREATE TABLE episode (
    id INTEGER,
    seasonid INTEGER NOT NULL,
    episode_ INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES _tblmedia (id) ON DELETE CASCADE,
    FOREIGN KEY(seasonid) REFERENCES _tblseason (id) ON DELETE CASCADE
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

    FOREIGN KEY(media_id) REFERENCES _tblmedia (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(library_id) REFERENCES library(id) ON DELETE CASCADE
);

CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    prefs BLOB NOT NULL DEFAULT '{}',
    claimed_invite TEXT NOT NULL UNIQUE,
    roles TEXT[] NOT NULL DEFAULT 'User',
    picture INTEGER UNIQUE,

    FOREIGN KEY(claimed_invite) REFERENCES invites(id),
    FOREIGN KEY(picture) REFERENCES assets(id)
);

CREATE TABLE progress (
    id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    delta INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    populated INTEGER NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY(media_id) REFERENCES _tblmedia (id) ON DELETE CASCADE ON UPDATE CASCADE,
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
    FOREIGN KEY (media_id) REFERENCES _tblmedia(id) ON DELETE CASCADE,
    FOREIGN KEY (genre_id) REFERENCES genre(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX genre_media_idx ON genre_media(genre_id, media_id);

CREATE TABLE invites (
    id TEXT PRIMARY KEY NOT NULL UNIQUE,
    date_added INTEGER NOT NULL
);

CREATE TABLE assets (
    id INTEGER PRIMARY KEY,
    remote_url TEXT UNIQUE,
    local_path TEXT NOT NULL UNIQUE,
    file_ext TEXT NOT NULL
);

CREATE TABLE media_posters (
    id INTEGER PRIMARY KEY,
    media_id INTEGER NOT NULL,
    asset_id INTEGER NOT NULL,

    FOREIGN KEY (media_id) REFERENCES _tblmedia(id) ON DELETE CASCADE,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX media_posters_idx ON media_posters(media_id, asset_id);

CREATE TABLE media_backdrops (
    id INTEGER PRIMARY KEY,
    media_id INTEGER NOT NULL,
    asset_id INTEGER NOT NULL,

    FOREIGN KEY (media_id) REFERENCES _tblmedia(id) ON DELETE CASCADE,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX media_backdrops_idx ON media_backdrops(media_id, asset_id);
