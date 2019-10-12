-- procedure to generate index
CREATE PROCEDURE drop_all_data()
LANGUAGE SQL
AS $$
DELETE FROM media;
DELETE FROM library;
DELETE FROM mediafile;
DELETE FROM episode;
DELETE FROM season;
DELETE FROM tv_show;
DELETE FROM movie;
DELETE FROM streamable_media;
$$;
