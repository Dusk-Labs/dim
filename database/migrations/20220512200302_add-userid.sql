ALTER TABLE users RENAME TO old_users;
ALTER TABLE progress RENAME TO old_progress;

CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    prefs BLOB NOT NULL DEFAULT '{}',
    claimed_invite TEXT NOT NULL UNIQUE,
    roles TEXT[] NOT NULL DEFAULT 'User',
    picture INTEGER UNIQUE,

    FOREIGN KEY(claimed_invite) REFERENCES invites(id),
    FOREIGN KEY(picture) REFERENCES assets(id)
);

INSERT INTO users (username, password, prefs, claimed_invite, roles, picture) SELECT * FROM old_users;

CREATE TABLE progress (
    id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    delta INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    populated INTEGER NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY(media_id) REFERENCES _tblmedia (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO progress (id, user_id, delta, media_id, populated)
SELECT op.id, u.id, op.delta, op.media_id, op.populated
FROM old_progress op
JOIN users u
ON op.user_id=u.username;

DROP TABLE old_users;
DROP TABLE old_progress;
