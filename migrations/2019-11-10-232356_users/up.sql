CREATE TYPE roles AS ENUM ('Owner', 'User');
CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    roles TEXT[] NOT NULL DEFAULT '{"User"}'
);

INSERT INTO users
VALUES (
    'user',
    '$aragon2i$m=4096,t=4,p=8$c0664f7543c7c0a4fb7ddbc07aa86539209cf3b444ab3a4b7a7bcf15f9e21836',
    '{"Owner"}'
);
