DROP INDEX media_excl_ep_idx;

-- Recreate media view
DROP VIEW media;

CREATE VIEW media AS
SELECT _tblmedia.*, pp.local_path as poster_path, bp.local_path as backdrop_path
FROM _tblmedia
LEFT OUTER JOIN assets pp ON _tblmedia.poster = pp.id
LEFT OUTER JOIN assets bp ON _tblmedia.backdrop = bp.id;

CREATE TRIGGER media_delete
INSTEAD OF DELETE ON media
BEGIN DELETE FROM _tblmedia WHERE _tblmedia.id = old.id; END;

-- Recreate season view
DROP VIEW season;

CREATE VIEW season AS
SELECT _tblseason.id, _tblseason.season_number, _tblseason.tvshowid, _tblseason.added, assets.local_path as poster
FROM _tblseason
LEFT OUTER JOIN assets ON _tblseason.poster = assets.id;

CREATE TRIGGER season_delete
INSTEAD OF DELETE ON season
BEGIN DELETE FROM _tblseason WHERE _tblseason.id = old.id; END;
