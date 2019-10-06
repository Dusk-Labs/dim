-- Library table
CREATE TABLE library (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    media_type VARCHAR(50) NOT NULL
);

-- Media table
-- This table contains the template for
-- the movie and tv shows tables minus containing
-- the paths because movies are streamable while
-- tv shows generally arent
-- The Episodes table will also inherit from here
CREATE TABLE media (
    id SERIAL,
    library_id INTEGER NOT NULL,

    name VARCHAR(80) NOT NULL,
    description TEXT,
    rating INTEGER,
    year INTEGER,
    added TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    media_type VARCHAR(50),
    genres TEXT[], -- NOTE: Use a separate table for genres
    PRIMARY KEY (id),
    CONSTRAINT fk_library FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE CASCADE
);

-- Streamble Media Table
-- This table contains the template for
-- Media that has a file attached to it
-- ie it can be streamed.
-- Currently only movies and episodes inherit from this
CREATE TABLE streamable_media (
    id SERIAL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES media (id) ON DELETE CASCADE
);

CREATE TABLE movie (
    id SERIAL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES streamable_media (id) ON DELETE CASCADE
);

CREATE TABLE tv_show (
    id SERIAL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES media (id) ON DELETE CASCADE
);

CREATE TABLE season (
    id SERIAL,
    season_number INTEGER NOT NULL,
    tvshowid INTEGER NOT NULL,
    added TEXT,
    poster TEXT,
    PRIMARY KEY (id),
    FOREIGN KEY(tvshowid) REFERENCES tv_show (id) ON DELETE CASCADE
);

CREATE TABLE episode (
    id SERIAL,
    seasonid INTEGER NOT NULL,
    episode_ INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY(id) REFERENCES streamable_media (id) ON DELETE CASCADE,
    FOREIGN KEY(seasonid) REFERENCES season (id) ON DELETE CASCADE
);

CREATE TABLE mediafile (
    id SERIAL,
    media_id INTEGER, -- Optional, populated on metadata search
    library_id INTEGER NOT NULL,
    target_file TEXT NOT NULL UNIQUE,

    raw_name TEXT NOT NULL,
    raw_year INTEGER,

    quality VARCHAR(255),
    codec VARCHAR(255),
    container VARCHAR(255),
    audio VARCHAR(255),
    original_resolution VARCHAR(255),
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

CREATE TABLE progress (
    id SERIAL,
    delta INTEGER,
    media_id INTEGER,

    PRIMARY KEY (id),
    FOREIGN KEY(media_id) REFERENCES media (id) ON DELETE CASCADE ON UPDATE CASCADE
);
