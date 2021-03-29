import React from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";

import "./QuickSearchResults.scss";

function Search(props) {
  let results;

  // SEARCH_START
  if (!props.results.fetched && !props.results.error) {
    results = (
      <div className="state showAfter100ms">
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
  if (props.results.fetched && !props.results.error) {
    const list = props.results.items.map((
      { name, id }, i
    ) => (
      <Link to={`/media/${id}`} key={i}>
        {name}
      </Link>
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
