import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { updateGlobalSettings } from "../../../actions/settings";
import Button from "../../../Components/Misc/Button";

import Field from "../../Auth/Field";

function DirectoryPaths() {
  const dispatch = useDispatch();
  const settings = useSelector((store) => store.settings);

  const [cache, setCache] = useState("");
  const [cacheErr, setCacheErr] = useState("");

  const [metadata, setMetadata] = useState("");
  const [metadataErr, setMetadataErr] = useState("");

  const updateCache = useCallback(() => {
    if (cache.length <= 1) {
      setCacheErr("Invalid path");
      return;
    }

    dispatch(
      updateGlobalSettings({
        cache_dir: cache,
      })
    );
  }, [cache, dispatch]);

  const cancelUpdateCache = useCallback(() => {
    setCache(settings.globalSettings.data.cache_dir);
  }, [settings.globalSettings.data.cache_dir]);

  const updateMetaData = useCallback(() => {
    if (metadata.length <= 1) {
      setMetadataErr("Invalid path");
      return;
    }

    dispatch(
      updateGlobalSettings({
        metadata_dir: metadata,
      })
    );
  }, [dispatch, metadata]);

  const cancelUpdateMetaData = useCallback(() => {
    setMetadata(settings.globalSettings.data.metadata_dir);
  }, [settings.globalSettings.data.metadata_dir]);

  useEffect(() => {
    const { data } = settings.globalSettings;

    setCache(data.cache_dir);
    setMetadata(data.metadata_dir);
  }, [settings]);

  return (
    <section>
      <h2>Directory paths</h2>
      <Field
        name="Cache"
        data={[cache, setCache]}
        error={[cacheErr, setCacheErr]}
      />
      {cache !== settings.globalSettings.data.cache_dir && (
        <div className="options">
          <Button onClick={updateCache}>Update</Button>
          <Button onClick={cancelUpdateCache} type="secondary">
            Cancel
          </Button>
        </div>
      )}
      <Field
        name="Metadata"
        data={[metadata, setMetadata]}
        error={[metadataErr, setMetadataErr]}
      />
      {metadata !== settings.globalSettings.data.metadata_dir && (
        <div className="options">
          <Button onClick={updateMetaData}>Update</Button>
          <Button onClick={cancelUpdateMetaData} type="secondary">
            Cancel
          </Button>
        </div>
      )}
    </section>
  );
}

export default DirectoryPaths;
