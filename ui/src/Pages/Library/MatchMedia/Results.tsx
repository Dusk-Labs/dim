import { useState, useEffect } from "react";
import SearchResult from "./SearchResult";
import { useExternalSearchQuery } from "api/v1/searchExternal";

export interface Props {
  query: string;
  params: Array<Record<string, any>>;
}

export const SearchResults = ({ query, params }: Props) => {
  const [skip, setSkip] = useState<boolean>(true);
  const [mediaType, setMediaType] = useState<string | null>(null);
  const [year, setYear] = useState<string | null>(null);
  const { data } = useExternalSearchQuery(
    { query, year, mediaType: mediaType ?? "" },
    { refetchOnMountOrArgChange: true, skip }
  );

  useEffect(() => {
    if (params.length === 0) return;

    setYear(null);

    for (const param of params) {
      if (param.name === "Year") {
        setYear(param.content.toString());
      }

      if (param.name === "Media") {
        setMediaType(param.content);
      }
    }
  }, [params, setMediaType, setYear]);

  useEffect(() => {
    if (!query) return;

    if (mediaType) {
      setSkip(false);
    }
  }, [mediaType, year, query, setSkip]);

  const results = !data
    ? []
    : data.map(
        ({
          overview,
          genres,
          title,
          rating,
          id,
          poster_path,
          year,
          duration,
        }) => {
          return (
            <SearchResult
              description={overview || ""}
              title={title}
              year={year?.toString() ?? null}
              rating={rating?.toString() ?? null}
              duration={duration ?? null}
              id={id}
              genres={genres || []}
              poster={poster_path || ""}
              media_type={mediaType!}
            />
          );
        }
      );

  return <>{results ? results : null}</>;
};

export default SearchResults;
