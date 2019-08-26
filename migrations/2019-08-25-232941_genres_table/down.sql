DROP TABLE genre_media;
DROP TABLE genre;

ALTER TABLE media ADD COLUMN genres TEXT[];
