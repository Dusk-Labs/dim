import { useEffect, useRef, useState, useCallback } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { connect } from "react-redux";
import { useHistory, withRouter } from "react-router-dom";

import QuickSearchResults from "./QuickSearchResults";
import { quickSearch } from "../../actions/search.js";

import "./Search.scss";

function Search(props) {
  const history = useHistory();

  const searchBox = useRef(null);
  const inputBox = useRef(null);

  const [query, setQuery] = useState("");
  const [showResults, setShowResults] = useState(false);

  const { quickSearch, auth } = props;

  const handleClick = useCallback(e => {
    if (showResults && searchBox.current) {
      if (searchBox.current.contains(e.target)) return;
      setShowResults(false);
    }
  }, [showResults]);

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    }
  }, [handleClick]);

  const handleOnChange = useCallback(e => {
    setQuery(e.target.value);
    setShowResults(e.target.value.length > 1);

    if (e.target.value.length > 1) {
      quickSearch(e.target.value, auth.token);
    }
  }, [auth.token, quickSearch]);

  const onKeyDown = useCallback((e) => {
    if (e.keyCode === 13) {
      history.push({
        pathname: "/search",
        search: `?query=${encodeURIComponent(query)}`
      });

      setQuery("");
      setShowResults(false);
    }
  }, [history, query]);

  const fullSearch = useCallback(() => {
    if (query.length >= 1) {
      history.push({
        pathname: "/search",
        search: `?query=${encodeURIComponent(query)}`
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
          value={query}
          onKeyDown={onKeyDown}
          onChange={handleOnChange}
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="off"
          spellCheck="false"
          placeholder="Search"
        />
        <button onClick={fullSearch}>
          <FontAwesomeIcon icon="search"/>
        </button>
      </div>
      {showResults && <QuickSearchResults/>}
    </div>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
});

const mapActionsToProps = {
  quickSearch
};

export default connect(mapStateToProps, mapActionsToProps)(withRouter(Search));
