import { useContext, useEffect, useState } from "react";
import { useParams } from "react-router";

import Folders from "./Folders";
import Files from "./Files";
import Search from "./Search";
import Options from "./Options";

import { SelectUnmatchedContext } from "./Context";
import { LibraryContext } from "../Context";
import { useSelector } from "react-redux";

import "./Index.scss";

const UnmatchedMedia = (props) => {
  const params = useParams();
  const { showUnmatched, unmatched } = useContext(LibraryContext);

  const libraries = useSelector(store => (
    store.library.fetch_libraries.items
  ));

  const [selectedFiles, setSelectedFiles] = useState({});
  const [currentFolder, setCurrentFolder] = useState();
  const [query, setQuery] = useState("");
  const [tmdbResults, setTmdbResults] = useState([]);
  const [mediaType, setMediaType] = useState();
  const [tmdbID, setTmdbID] = useState();
  const [error, setError] = useState("");

  useEffect(() => {
    setError("");
  }, [selectedFiles, currentFolder, query, tmdbResults, mediaType, tmdbID]);

  useEffect(() => {
    setMediaType(
      libraries.filter(lib => lib.id === parseInt(params.id))[0].media_type
    );
  }, [libraries, params.id]);

  useEffect(() => {
    if (Object.keys(selectedFiles).length === 0) {
      setTmdbID();
      setTmdbResults([]);
      setError();
      setQuery();
    }
  }, [selectedFiles]);

  useEffect(() => {
    if (showUnmatched) return;

    setTmdbID();
    setTmdbResults([]);
    setError();
    setQuery();
    setSelectedFiles({});
    setCurrentFolder();
  }, [showUnmatched]);

  const initialValue = {
    setManuallyMatch: props.setManuallyMatch,
    selectedFiles, setSelectedFiles,
    currentFolder, setCurrentFolder,
    mediaType, setMediaType,
    tmdbResults, setTmdbResults,
    query, setQuery,
    tmdbID, setTmdbID
  };

  const count = Object.values(unmatched.items).flat().length;

  return (
    <SelectUnmatchedContext.Provider value={initialValue}>
      <div className={`unmatchedMedia show-${showUnmatched}`}>
        <h2>Unmatched</h2>
        <p className="sectionDesc">Could not find an accurate match for {count} {count === 0 ? "file" : "files"} in this library.</p>
        <div className="selectUnmatchedMedia">
          {unmatched.fetched && (
            <Folders/>
          )}
          {currentFolder && (
            <Files/>
          )}
          {(Object.values(selectedFiles).length > 0) && <Search/>}
          {error && <p className="err">Error: {error}</p>}
          <Options/>
        </div>
      </div>
    </SelectUnmatchedContext.Provider>
  );
};

export default UnmatchedMedia;
