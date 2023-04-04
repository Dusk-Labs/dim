-- Adds a delete trigger to both media_posters and media_backdrops which
-- ensure that the asset erased is also deleted from the assets table
CREATE TRIGGER IF NOT EXISTS media_backdrops_propagate
AFTER DELETE ON media_backdrops
FOR EACH ROW BEGIN
    DELETE FROM assets WHERE id = old.asset_id;
END;

CREATE TRIGGER IF NOT EXISTS media_posters_propagate
AFTER DELETE ON media_posters
FOR EACH ROW BEGIN
    DELETE FROM assets WHERE id = old.asset_id;
END;
