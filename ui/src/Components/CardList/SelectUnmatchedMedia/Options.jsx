import { useCallback, useContext } from "react";
import { useSelector } from "react-redux";
import { SelectUnmatchedContext } from "./Context";

import "./Options.scss";

const SelectUnmatchedMediaOptions = () => {
  const { mediaType, tmdbID, selectedFiles, setError, setManuallyMatch } = useContext(SelectUnmatchedContext);

  const { token } = useSelector(store => ({
    token: store.auth.token
  }));

  const match = useCallback(async () => {
    if (!tmdbID || !mediaType || Object.keys(selectedFiles).length === 0) return;

    const files = Object.values(selectedFiles);

    if (files.length === 0) return;

    const config = {
      method: "PATCH",
      headers: {
        "authorization": token
      }
    };

    for (const file of files) {
      console.log(`[Matcher] matching ${file.id} to tmdb ID ${tmdbID}`);

      const req = await fetch(`/api/v1/mediafile/${file.id}/match?tmdb_id=${tmdbID}&media_type=${mediaType}`, config);

      if (req.status !== 200) {
        setError(req.statusText);
        return;
      }
    }

    console.log("[Matcher] finished");
  }, [mediaType, selectedFiles, setError, tmdbID, token]);

  const close = useCallback(async () => {
    setManuallyMatch(false);
  }, [setManuallyMatch]);

  const totalCount = Object.keys(selectedFiles).length;

  return (
    <div className="selectUnmatchedMediaOptions">
      <button onClick={close} className="cancelBtn">Cancel</button>
      <button onClick={() => match()} className={`disabled-${!mediaType || !tmdbID || totalCount === 0}`}>
        Match selected media {totalCount ? `(${totalCount})` : ""}
      </button>
    </div>
  );
};

export default SelectUnmatchedMediaOptions;
