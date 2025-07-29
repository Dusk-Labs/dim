import { useCallback, useContext, useState } from "react";
import { useSelector } from "react-redux";
import DimLogo from "../../assets/DimLogo";
import SearchIcon from "../../assets/Icons/Search";
import CheckIcon from "../../assets/Icons/Check";
import ImageLoad from "../../Components/ImageLoad";

import { RematchContext } from "./Context";

import "./Search.scss";

const SelectRematchSearch = () => {
  const { token } = useSelector((store) => ({
    token: store.auth.token,
  }));

  const {
    mediaType,
    query,
    setQuery,
    tmdbID,
    setTmdbID,
    tmdbResults,
    setTmdbResults,
  } = useContext(RematchContext);

  const [fetched, setFetched] = useState(false);

  const search = useCallback(() => {
    if (!query) {
      setTmdbResults([]);
      setFetched(false);
      setTmdbID();

      return;
    }

    if (!mediaType) return;

    setFetched(false);
    setTmdbID();

    (async () => {
      const config = {
        headers: {
          authorization: token,
        },
      };

      const req = await fetch(
        `/api/v1/media/tmdb_search?query=${query}&media_type=${mediaType}`,
        config
      );

      if (req.status !== 200) {
        return;
      }

      const payload = await req.json();

      setFetched(true);
      setTmdbResults(payload);
    })();
  }, [mediaType, query, setTmdbID, setTmdbResults, token]);

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "Enter") {
        search();
      }
    },
    [search]
  );

  const selectTmdb = useCallback(
    (id) => {
      tmdbID === id ? setTmdbID() : setTmdbID(id);
    },
    [setTmdbID, tmdbID]
  );

  return (
    <div className="selectUnmatchedMediaSearch">
      <p className="desc">
        Search for a {mediaType === "movie" ? "movie" : "show"} to match to:
      </p>
      <div className="searchField">
        <input
          onKeyDown={handleKeyDown}
          value={query ? query : ""}
          onChange={(e) => setQuery(e.target.value)}
        />
        <button onClick={search}>
          <SearchIcon />
        </button>
      </div>
      {fetched && tmdbResults.length === 0 && (
        <p className="err">No results found</p>
      )}
      {tmdbResults.length > 0 && (
        <div className="tmdbResults">
          {tmdbResults.map((result, i) => (
            <div
              onClick={() => selectTmdb(result.id)}
              className={`resultCard selected-${
                result.id === tmdbID || tmdbID === undefined
              }`}
              key={i}
            >
              <div className="tmdbImageWrapper">
                <ImageLoad
                  src={result.poster_path}
                  triggerAnimation="onHideImage"
                >
                  {({ imageSrc, loaded, error, setErr }) => (
                    <>
                      {loaded && !error && (
                        <img
                          src={imageSrc}
                          alt="cover"
                          onError={() => setErr(true)}
                        />
                      )}
                      {error && (
                        <div className="placeholder">
                          <DimLogo />
                        </div>
                      )}
                    </>
                  )}
                </ImageLoad>
                <div className={`selectedBox selected-${result.id === tmdbID}`}>
                  <CheckIcon />
                </div>
              </div>
              <p>{result.title}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default SelectRematchSearch;
