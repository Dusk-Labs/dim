import { useCallback, useContext, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { useParams } from "react-router";

import Folders from "./Folders";
import Files from "./Files";
import Search from "./Search";
import Options from "./Options";

import MediaTypeSelection from "../../../Modals/NewLibrary/MediaTypeSelection";
import { SelectUnmatchedContext } from "./Context";
import { LibraryContext } from "../Context";

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

  const [filesMatched, setFilesMatched] = useState([]);
  const [matching, setMatching] = useState(false);

  const clearData = useCallback(() => {
    setTmdbID();
    setTmdbResults([]);
    setError();
    setQuery();
    setSelectedFiles({});
    setCurrentFolder();
  }, []);

  useEffect(() => {
    setError("");
  }, [selectedFiles, currentFolder, query, tmdbResults, mediaType, tmdbID]);

  useEffect(() => {
    const [library] = libraries.filter(lib => lib.id === parseInt(params.id));

    if(library !== undefined)
      setMediaType(library.mediaType);
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
    clearData();
  }, [clearData, showUnmatched]);

  const initialValue = {
    setManuallyMatch: props.setManuallyMatch,
    selectedFiles, setSelectedFiles,
    currentFolder, setCurrentFolder,
    mediaType, setMediaType,
    tmdbResults, setTmdbResults,
    query, setQuery,
    tmdbID, setTmdbID,
    filesMatched, setFilesMatched,
    matching, setMatching,
    clearData
  };

  const count = Object.values(unmatched.items).flat().length;

  return (
    <SelectUnmatchedContext.Provider value={initialValue}>
      <div className={`unmatchedMedia show-${showUnmatched}`}>
        {matching && (
          <div className="matchingProgress">
            <div className="progress" style={{width: `${(filesMatched.length / Object.keys(selectedFiles).length) * 100}%`}}/>
            {filesMatched.length === Object.keys(selectedFiles).length
              ? <p>Finished matching</p>
              : <p>({filesMatched.length}/{Object.keys(selectedFiles).length}) Matching '{filesMatched[filesMatched.length - 1]}'</p>
            }
          </div>
        )}
        <h2>Unmatched</h2>
        <p className="sectionDesc">Could not find an accurate match for {count} {count === 0 ? "file" : "files"} in this library.</p>
        <div className="selectUnmatchedMedia">
          {unmatched.fetched && (
            <Folders/>
          )}
          {currentFolder && (
            <Files/>
          )}
          {Object.values(selectedFiles).length > 0 && (
            <div>
              <MediaTypeSelection mediaType={mediaType} setMediaType={setMediaType}/>
            </div>
          )}
          {Object.values(selectedFiles).length > 0 && <Search/>}
          {error && <p className="err">Error: {error}</p>}
          <Options/>
        </div>
      </div>
    </SelectUnmatchedContext.Provider>
  );
};

export default UnmatchedMedia;
