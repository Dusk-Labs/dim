import { useCallback, useContext } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useParams } from "react-router";

import { fetchLibraryUnmatched } from "../../../actions/library";
import Button from "../../../Components/Misc/Button";
import { LibraryContext } from "../Context";
import { SelectUnmatchedContext } from "./Context";
import { NOTIFICATIONS_ADD } from "../../../actions/types";

import "./Options.scss";

const SelectUnmatchedMediaOptions = () => {
  const dispatch = useDispatch();
  const params = useParams();

  const { setShowUnmatched } = useContext(LibraryContext);
  const { mediaType, tmdbID, selectedFiles, clearData, setError, setFilesMatched, setMatching } = useContext(SelectUnmatchedContext);

  const { token } = useSelector(store => ({
    token: store.auth.token
  }));

  const match = useCallback(async () => {
    if (!tmdbID || !mediaType || Object.keys(selectedFiles).length === 0) return;

    const files = Object.values(selectedFiles);

    if (files.length === 0) return;

    setMatching(true);

    const config = {
      method: "PATCH",
      headers: {
        "authorization": token
      }
    };

    for (const file of files) {
      setFilesMatched(state => [...state, file.name]);

      const req = await fetch(`/api/v1/mediafile/${file.id}/match?tmdb_id=${tmdbID}&media_type=${mediaType}`, config);

      if (req.status !== 200) {
        setError(req.statusText);
        return;
      }
    }

    dispatch({
      type: NOTIFICATIONS_ADD,
      payload: {
        msg: `Sucessfuly matched ${files.length} files.`
      }
    });

    clearData();
    dispatch(fetchLibraryUnmatched(params.id));
  }, [clearData, dispatch, mediaType, params.id, selectedFiles, setError, setFilesMatched, setMatching, tmdbID, token]);

  const totalCount = Object.keys(selectedFiles).length;

  return (
    <div className="selectUnmatchedMediaOptions">
      <Button
        type="secondary"
        onClick={() => setShowUnmatched(false)}
      >
        Close
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
