ALTER TABLE _tblmedia RENAME TO old_media;

CREATE TABLE _tblmedia (
    id INTEGER PRIMARY KEY,
    library_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    description TEXT,
    rating INTEGER,
    year INTEGER,
    added TEXT,
    poster INTEGER,
    backdrop INTEGER,
    media_type TEXT NOT NULL,

    FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE CASCADE,
    FOREIGN KEY (poster) REFERENCES assets(id),
    FOREIGN KEY (backdrop) REFERENCES assets(id)
);

INSERT INTO _tblmedia (id, library_idm name, description, rating, year, added, poster, backdrop, media_type) SELECT * FROM old_media;
DROP TABLE old_media;
