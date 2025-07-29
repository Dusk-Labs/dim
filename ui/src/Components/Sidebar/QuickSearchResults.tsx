import { Link } from "react-router-dom";

import { useQuickSearchQuery } from "../../api/v1/search";

import "./QuickSearchResults.scss";

interface Props {
  query: string;
}

function Search({ query }: Props) {
  const { data: items, error, isFetching } = useQuickSearchQuery(query);

  let resultsSection;

  if (isFetching) {
    resultsSection = (
      <div className="state showAfter100ms">
        <p>Loading</p>
      </div>
    );
  } else if (error) {
    resultsSection = (
      <div className="state">
        <p>Cannot load data</p>
      </div>
    );
  } else {
    const list = (items || []).map(({ name, id }, i) => (
      <Link to={`/media/${id}`} key={i}>
        {name}
      </Link>
    ));

    resultsSection = (
      <div className="results">
        <p>Results - {list.length}</p>
        {list.length > 0 && <div className="result-list">{list}</div>}
      </div>
    );
  }

  return <div className="quickSearchResults">{resultsSection}</div>;
}

export default Search;
