import { useCallback, useContext } from "react";
import { useSelector } from "react-redux";
import { useParams } from "react-router";
import Button from "../../../Components/Misc/Button";
import { LibraryContext } from "../Context";
import { SelectUnmatchedContext } from "./Context";

import "./Options.scss";

const SelectUnmatchedMediaOptions = () => {
  const { setShowUnmatched } = useContext(LibraryContext);
  const { mediaType, tmdbID, selectedFiles, setError } = useContext(SelectUnmatchedContext);

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

  const totalCount = Object.keys(selectedFiles).length;

  return (
    <div className="selectUnmatchedMediaOptions">
      <Button
        type="secondary"
        onClick={() => setShowUnmatched(false)}
      >
        Cancel
      </Button>
      <Button
        onClick={() => match()}
        disabled={!mediaType || !tmdbID || totalCount === 0}
      >
        Match selected media {totalCount ? `(${totalCount})` : ""}
      </Button>
    </div>
  );
};

export default SelectUnmatchedMediaOptions;
