CREATE TYPE roles AS ENUM ('Owner', 'User');
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    roles TEXT[] NOT NULL DEFAULT '{"User"}'
);
ALTER TABLE progress ADD COLUMN user_id TEXT NOT NULL;
ALTER TABLE progress ADD CONSTRAINT fk_uid FOREIGN KEY (user_id) REFERENCES users(username) ON DELETE CASCADE;
ALTER TABLE progress ADD CONSTRAINT u_constraint UNIQUE (user_id, media_id);
