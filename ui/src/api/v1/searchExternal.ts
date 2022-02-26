import v1 from "./index";

export interface SearchResult {
  id: number;
  title: string;
  year?: number;
  overview?: string;
  poster_path?: string;
  genres: Array<string>;
  rating?: number;
  duration?: string;
}

export const search = v1.injectEndpoints({
  endpoints: (build) => ({
    externalSearch: build.query<
      SearchResult[],
      { query: string; year: string | null; mediaType: string }
    >({
      query: ({ query, year, mediaType }) => {
        /// FIXME: This should be normalized upstream, or best yet, the server can just accept non-normalized versions.
        let normMediaType = mediaType === "Movies" ? "movie" : "tv";
        let baseQuery = `media/tmdb_search?query=${query}&media_type=${normMediaType}`;

        if (year) {
          baseQuery = `${baseQuery}&year=${year}`;
        }

        return baseQuery;
      },
    }),
  }),
});

export const { useExternalSearchQuery } = search;

export default search;
