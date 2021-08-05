import { useSelector } from "react-redux";
import { Link } from "react-router-dom";

import "./QuickSearchResults.scss";

function Search() {
  const results = useSelector(store => store.search.quick_search);

  let resultsSection;

  // SEARCH_START
  if (!results.fetched && !results.error) {
    resultsSection = (
      <div className="state showAfter100ms">
        <p>Loading</p>
      </div>
    );
  }

  // SEARCH_ERR
  if (results.fetched && results.error) {
    resultsSection = (
      <div className="state">
        <p>Cannot load data</p>
      </div>
    );
  }

  // SEARCH_OK
  if (results.fetched && !results.error) {
    const list = results.items.map((
      { name, id }, i
    ) => (
      <Link to={`/media/${id}`} key={i}>
        {name}
      </Link>
    ));

    resultsSection = (
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
      {resultsSection}
    </div>
  );
}

export default Search;
