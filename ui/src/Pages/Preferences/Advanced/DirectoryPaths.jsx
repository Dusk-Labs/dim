import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import Field from "../../Auth/Field";

function DirectoryPaths() {
  const dispatch = useDispatch();

  const settings = useSelector(store => store.settings);

  const [cache, setCache] = useState("");
  const [cacheErr, setCacheErr] = useState("");

  const [metadata, setMetadata] = useState("");
  const [metadataErr, setMetadataErr] = useState("");

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
      <div className="fields">
        <Field
          name="Metadata"
          data={[metadata, setMetadata]}
          error={[metadataErr, setMetadataErr]}
        />
      </div>
    </section>
  );
}

export default DirectoryPaths;
