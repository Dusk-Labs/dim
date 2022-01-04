import v1 from "./index";
import { SearchResult } from "./types";

export const search = v1.injectEndpoints({
  endpoints: (build) => ({
    search: build.query<SearchResult[], string>({
      query: (params) => `search${params}`
    }),
    quickSearch: build.query<SearchResult[], string>({
      query: (query) => `search?query=${query}&quick=true`
    })
  })
});

export const { useSearchQuery, useQuickSearchQuery } = search;

export default search;
