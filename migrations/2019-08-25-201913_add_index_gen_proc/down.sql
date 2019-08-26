DROP ROUTINE IF EXISTS drop_all_data();
DROP TRIGGER tr_trigger_generate_index ON media;
DROP FUNCTION trigger_generate_index();

ALTER TABLE media
DROP COLUMN name_search_index;
