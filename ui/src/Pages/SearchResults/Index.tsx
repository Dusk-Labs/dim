import { useLocation } from "react-router";

import { useSearchQuery } from "../../api/v1/search";
import PropCardList from "./PropCardList";

function SearchResults() {
  const location = useLocation();
  const search = location.search;
  const searchParams = new URLSearchParams(search);
  const query = searchParams.get("query");

  if (!query) {
    return (
      <div className="card_list">
        No search query. Use the search box in the sidebar to search for media.
      </div>
    );
  }

  return <SearchResultsInner query={query} search={search} />;
}

interface Props {
  query: string;
  search: string;
}

function SearchResultsInner({ query, search }: Props) {
  const title = `Dim - Query results for '${query}'`;
  const { data: items, error, isFetching } = useSearchQuery(search);

  document.title = title;

  return (
    <PropCardList
      error={error?.toString()}
      items={items}
      title={title}
      isFetching={isFetching}
    />
  );
}

export default SearchResults;
