import { useEffect, useRef, useState, useCallback } from "react";
import { useHistory } from "react-router-dom";

import QuickSearchResults from "./QuickSearchResults";
import SearchIcon from "../../assets/Icons/Search";

import "./Search.scss";

function Search() {
  const history = useHistory();

  const searchBox = useRef<HTMLDivElement>(null);
  const inputBox = useRef(null);

  const [query, setQuery] = useState<string | null>(null);
  const [showResults, setShowResults] = useState(false);

  const handleClick = useCallback((e) => {
    if (searchBox.current) {
      if (searchBox.current.contains(e.target)) {
        setShowResults(true);
      } else {
        setShowResults(false);
      }
    }
  }, []);

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    };
  }, [handleClick]);

  const handleOnChange = useCallback((e) => {
    setQuery(e.target.value);
    setShowResults(e.target.value.length > 1);
  }, []);

  const onKeyDown = useCallback(
    (e) => {
      if (query && query.length > 1 && e.keyCode === 13) {
        history.push({
          pathname: "/search",
          search: `?query=${encodeURIComponent(query || "")}`,
        });

        setQuery("");
        setShowResults(false);
      }
    },
    [history, query]
  );

  const fullSearch = useCallback(() => {
    if (query && query.length >= 1) {
      history.push({
        pathname: "/search",
        search: `?query=${encodeURIComponent(query)}`,
      });

      setQuery("");
      setShowResults(false);
    }
  }, [history, query]);

  return (
    <div className="search-box" ref={searchBox}>
      <div className="search-box-wrapper">
        <input
          ref={inputBox}
          value={query || ""}
          onKeyDown={onKeyDown}
          onChange={handleOnChange}
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="off"
          spellCheck="false"
          placeholder="Search"
          type="search"
        />
        <button onClick={fullSearch}>
          <SearchIcon />
        </button>
      </div>
      {query && showResults && <QuickSearchResults query={query} />}
    </div>
  );
}

export default Search;
