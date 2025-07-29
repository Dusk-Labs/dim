import v1 from "./index";
import { Media } from "./types";

/**
 * The results returned by the search API.
 */
export interface SearchResult extends Media {
  library_id: number;
}

export const search = v1.injectEndpoints({
  endpoints: (build) => ({
    search: build.query<SearchResult[], string>({
      query: (params) => `search${params}`,
    }),
    quickSearch: build.query<SearchResult[], string>({
      query: (query) => `search?query=${query}&quick=true`,
    }),
  }),
});

export const { useSearchQuery, useQuickSearchQuery } = search;

export default search;
