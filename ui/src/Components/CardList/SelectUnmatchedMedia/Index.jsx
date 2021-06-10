import { useEffect, useState } from "react";

import Folders from "./Folders";
import Files from "./Files";
import MediaTypeSelection from "./MediaTypeSelection";
import Search from "./Search";
import Options from "./Options";

import { SelectUnmatchedContext } from "./Context";

import "./Index.scss";

const SelectUnmatchedMedia = (props) => {
  const [selectedFiles, setSelectedFiles] = useState({});
  const [currentFolder, setCurrentFolder] = useState(Object.keys(props.items)[0]);
  const [query, setQuery] = useState("");
  const [tmdbResults, setTmdbResults] = useState([]);
  const [mediaType, setMediaType] = useState();
  const [tmdbID, setTmdbID] = useState();
  const [error, setError] = useState("");

  const { items } = props;

  useEffect(() => {
    setError("");
  }, [selectedFiles, currentFolder, query, tmdbResults, mediaType, tmdbID]);

  const initialValue = {
    setManuallyMatch: props.setManuallyMatch,
    selectedFiles, setSelectedFiles,
    currentFolder, setCurrentFolder,
    mediaType, setMediaType,
    tmdbResults, setTmdbResults,
    query, setQuery,
    tmdbID, setTmdbID,
    items
  };

  return (
    <SelectUnmatchedContext.Provider value={initialValue}>
      <div className="selectUnmatchedMedia">
        <Folders/>
        {items[currentFolder] && (
          <Files/>
        )}
        {Object.values(selectedFiles).length > 0 && (
          <MediaTypeSelection/>
        )}
        {(Object.values(selectedFiles).length > 0 && mediaType) && <Search/>}
        {error && <p className="err">Error: {error}</p>}
        <Options/>
      </div>
    </SelectUnmatchedContext.Provider>
  );
};

export default SelectUnmatchedMedia;
