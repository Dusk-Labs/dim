-- Your SQL goes here
CREATE TABLE invites (
    id SERIAL PRIMARY KEY,
    token TEXT NOT NULL UNIQUE
);
