import { useCallback, useState, useEffect } from "react";
import { useParams } from "react-router-dom";

import Cards from "./Cards";

import MatchMedia from "./MatchMedia/Index";
import { useGetUnmatchedMediaFilesQuery } from "api/v1/unmatchedMedia";

import "./Index.scss";

interface LibraryParams {
  id: string;
}

const useDebounced = (value: string, delay: number): string => {
  const [debouncedValue, setDebouncedValue] = useState(value);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
};

const Library = () => {
  const { id } = useParams<LibraryParams>();
  const [searchQuery, setSearchQuery] = useState<string>("");
  const debouncedSearchQuery = useDebounced(searchQuery, 75);

  const { data, refetch } = useGetUnmatchedMediaFilesQuery(
    { id: id, search: debouncedSearchQuery },
    { refetchOnMountOrArgChange: true }
  );

  const mediafileSearch = useCallback(
    (query: string) => {
      setSearchQuery(query);
    },
    [setSearchQuery]
  );

  return (
    <div className="library">
      {data && (data.count > 0 || debouncedSearchQuery.length > 0) && (
        <MatchMedia
          data={data}
          refetch={refetch}
          mediafileSearch={mediafileSearch}
        />
      )}
      <Cards />
    </div>
  );
};

export default Library;
