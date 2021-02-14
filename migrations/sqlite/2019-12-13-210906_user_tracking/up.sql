-- Your SQL goes here
CREATE TABLE invites (
    id INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE
);
