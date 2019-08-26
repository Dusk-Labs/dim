-- add search index col to table
ALTER TABLE media
ADD COLUMN name_search_index tsvector;
UPDATE media SET name_search_index = to_tsvector(name);

ALTER TABLE media
ALTER COLUMN name_search_index
SET NOT NULL;

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

CREATE FUNCTION trigger_generate_index() RETURNS trigger AS $$
            begin
              new.name_search_index :=
                 to_tsvector(new.name);
              return new;
            end
            $$ LANGUAGE plpgsql;

            CREATE TRIGGER tr_trigger_generate_index BEFORE INSERT OR UPDATE
            ON media
            FOR EACH ROW EXECUTE PROCEDURE trigger_generate_index();
