import React, { useEffect, useState } from "react";
import { connect } from "react-redux";
import { HashLink } from 'react-router-hash-link';

import "./QuickSearchResults.scss";

function Search(props) {
  /*
    to prevent container from appearing/dissapearing
    on every key press.
  */
  const [fetchedOnce, setFetchedOnce] = useState(false);

  useEffect(() => {
    if (!fetchedOnce) {
      setFetchedOnce(true);
    }
  }, [props.results.fetched])

  let results;

  // SEARCH_START
  if (props.results.fetching) {
    results = (
      <div className="state">
        <p>Loading</p>
      </div>
    );
  }

  // SEARCH_ERR
  if (props.results.fetched && props.results.error) {
    results = (
      <div className="state">
        <p>Cannot load data</p>
      </div>
    );
  }

  // SEARCH_OK
  if ((props.results.fetched || fetchedOnce) && !props.results.error) {
    const list = props.results.items.map((
      { name, library_id, id }, i
    ) => (
      <HashLink
        to={`/library/${library_id}#${id}`}
        scroll={elm => {
          elm.scrollIntoView({ behavior: "smooth", block: "center" });
          elm.style.animation = "cardGlow 3s ease-in-out infinite";
        }}
        key={i}
      >
        {name}
      </HashLink>
    ));

    results = (
      <div className="results">
        <p>Results - {list.length}</p>
        {list.length > 0 && (
          <div className="result-list">{list}</div>
        )}
      </div>
    );
  }

  return (
    <div className="quickSearchResults">
      {results}
    </div>
  );
}

const mapStateToProps = (state) => ({
  results: state.search.quick_search
});

export default connect(mapStateToProps)(Search);
