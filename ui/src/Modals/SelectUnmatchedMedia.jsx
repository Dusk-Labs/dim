import { useCallback, useEffect, useState } from "react";
import { useSelector } from "react-redux";

import ModalBox from "./Index";
import CardImage from "./Image";

import ArrowLeftIcon from "../assets/Icons/ArrowLeft";
import FilmIcon from "../assets/Icons/Film";
import TvIcon from "../assets/Icons/TvIcon";
import FolderIcon from "../assets/Icons/Folder";

import "./SelectUnmatchedMedia.scss";

const SelectUnmatchedMedia = (props) => {
  const [selectedFiles, setSelectedFiles] = useState({});
  const [currentFolder, setCurrentFolder] = useState("");
  const [continueWithSelected, setContinueWithSelected] = useState(false);
  const [query, setQuery] = useState("");
  const [tmdbResults, setTmdbResults] = useState([]);
  const [mediaType, setMediaType] = useState("movie");
  const [tmdbID, setTmdbID] = useState();
  const [error, setError] = useState("");

  const { token } = useSelector(store => ({
    token: store.auth.token
  }));

  const selectMovie = useCallback(() => {
    if (mediaType !== "movie") {
      setMediaType("movie");
    }
  }, [mediaType, setMediaType]);

  const selectTv = useCallback(() => {
    if (mediaType !== "tv") {
      setMediaType("tv");
    }
  }, [mediaType, setMediaType]);

  const selectFolder = useCallback((folder) => {
    if (currentFolder === folder) return;
    setCurrentFolder(folder);
  }, [currentFolder]);

  const selectFile = useCallback((file) => {
    console.log(file);
    setSelectedFiles(state => ({
      ...state,
      [file.name]: state[file.name] ? null : {
        id: file.id,
        parent: currentFolder
      }
    }));
  }, [currentFolder]);

  const selectAll = useCallback(() => {
    for (const file of props.unmatched[currentFolder]) {
      setSelectedFiles(state => ({
        ...state,
        [file.name]: {
          id: file.id,
          parent: currentFolder
        }
      }));
    }
  }, [currentFolder, props.unmatched]);

  const search = useCallback(() => {
    if (!query) return;

    (async () => {
      const config = {
        headers: {
          "authorization": token
        }
      };

      const req = await fetch(`/api/v1/media/tmdb_search?query=${query}&media_type=${mediaType}`, config);

      if (req.status !== 200) {
        return;
      }

      const payload = await req.json();

      setTmdbResults(payload);
    })();
  }, [mediaType, query, token]);

  const handleKeyDown = useCallback((e) => {
    if (e.key === "Enter") {
      search();
    }
  }, [search]);

  const selectTmdb = useCallback((id) => {
    tmdbID === id ? setTmdbID() : setTmdbID(id);
  }, [tmdbID]);

  const match = useCallback(async (closeModal) => {
    if (!tmdbID || !mediaType) return;

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
  }, [mediaType, selectedFiles, tmdbID, token]);

  useEffect(() => {
    setError("");
  }, [selectedFiles, currentFolder, continueWithSelected, query, tmdbResults, mediaType, tmdbID]);

  const clean = useCallback(() => {
    setSelectedFiles({});
    setCurrentFolder("");
    setContinueWithSelected(false);
    setQuery("");
    setTmdbResults([]);
    setMediaType("movie");
    setTmdbID();
    setError("");
  }, []);

  return (
    <ModalBox
      id="modalSelectMediaVersion"
      activatingComponent={props.children}
      cleanUp={clean}
    >
      {closeModal => (
        <div className="modalSelectUnmatchedMedia">
          {!continueWithSelected
            ? <h3>Select media to match</h3>
            : <h3>Select what {mediaType === "movie" ? "movie" : "show"} your media belongs to</h3>
          }
          <div className="separator"/>
          {!continueWithSelected && (
            <div className="foldersWrapper">
              <div className="folders">
                {Object.keys(props.unmatched).map((folder, i) => {
                  const files = Object.values(selectedFiles);
                  const count = files.filter(file => file?.parent === folder).length;

                  return (
                    <div
                      key={i}
                      className={`folder selected-${currentFolder === folder}`}
                      onClick={() => selectFolder(folder)}
                    >
                      <div className="folderIcon">
                        <FolderIcon/>
                        {count > 0 && <p>{count}</p>}
                      </div>
                      <p>{folder}</p>
                    </div>
                  );
                })}
              </div>
              {props.unmatched[currentFolder] && (
                <>
                  <button className="selectAll" onClick={selectAll}>Select all files</button>
                  <div className="mediaWrapper">
                    <div className="media">
                      {props.unmatched[currentFolder].map((file, i) => (
                        <div
                          key={i}
                          className={`file selected-${!!selectedFiles[file.name]}`}
                          onClick={() => selectFile(file)}
                        >
                          <div className="selectBox"/>
                          <p>{file.name}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                  {Object.values(selectedFiles).length > 0 && (
                    <div className="mediaTypeSelection">
                      <p>Selected media belongs to a:</p>
                      <div className="types">
                        <div className="type" onClick={selectMovie}>
                          <FilmIcon/>
                          <p>Movie</p>
                          <div className={`select ${mediaType === "movie"}`}/>
                        </div>
                        <div className="type" onClick={selectTv}>
                          <TvIcon/>
                          <p>Show</p>
                          <div className={`select ${mediaType === "tv"}`}/>
                        </div>
                      </div>
                    </div>
                  )}
                </>
              )}
            </div>
          )}
          {continueWithSelected && (
            <div className="searchTmdb">
              <input
                onKeyDown={handleKeyDown}
                value={query}
                placeholder={`Search for ${mediaType === "movie" ? "movies" : "shows"}`}
                onChange={e => setQuery(e.target.value)}
              />
              <div className="tmdbResults">
                {tmdbResults.map((result, i) => (
                  <div
                    onClick={() => selectTmdb(result.id)}
                    className={`resultCard selected-${result.id === tmdbID || tmdbID === undefined}`}
                    key={i}
                  >
                    <CardImage src={result.poster_path}/>
                    <p>{result.title}</p>
                  </div>
                ))}
              </div>
            </div>
          )}
          {error && <p className="err">Error: {error}</p>}
          <div className="options">
            {continueWithSelected && (
              <button onClick={() => setContinueWithSelected(false)}>
                <ArrowLeftIcon/>
              </button>
            )}
            <div className="btns">
              <button className="cancelBtn" onClick={closeModal}>Cancel</button>
              {continueWithSelected
                ? <button onClick={() => match(closeModal)}>Match media</button>
                : <button onClick={() => setContinueWithSelected(true)}>Continue with selected</button>
              }
            </div>
          </div>
        </div>
      )}
    </ModalBox>
  );
};

export default SelectUnmatchedMedia;
