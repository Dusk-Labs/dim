import { useCallback, useContext, useEffect, useState } from "react";
import { useSelector } from "react-redux";

import CardImage from "../../../Modals/Image";
import { SelectUnmatchedContext } from "./Context";

import "./Search.scss";

const SelectUnmatchedMediaSearch = () => {
  const { token } = useSelector(store => ({
    token: store.auth.token
  }));

  const { mediaType, query, setQuery, tmdbID, setTmdbID, tmdbResults, setTmdbResults} = useContext(SelectUnmatchedContext);

  const [prevMediaType, setPrevMediaType] = useState("");

  const search = useCallback(() => {
    if (!query) return;

    setTmdbResults([]);
    setTmdbID();

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
  }, [mediaType, query, setTmdbID, setTmdbResults, token]);

  useEffect(() => {
    if (mediaType === prevMediaType) return;

    setPrevMediaType(mediaType);
    search();
  }, [mediaType, prevMediaType, search]);

  const handleKeyDown = useCallback((e) => {
    if (e.key === "Enter") {
      search();
    }
  }, [search]);

  const selectTmdb = useCallback((id) => {
    tmdbID === id ? setTmdbID() : setTmdbID(id);
  }, [setTmdbID, tmdbID]);

  return (
    <div className="selectUnmatchedMediaSearch">
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
  );
};

export default SelectUnmatchedMediaSearch;
