CREATE TYPE roles AS ENUM ('Owner', 'User');
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    roles TEXT[] NOT NULL DEFAULT '{"User"}'
);
