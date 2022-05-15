import { createContext } from "react";

export interface ISearchResultContext {
  // Currently selected search result
  current: number | null;
  // Alter currently selected result
  setCurrent: (current: number | null) => void;
  // Callback for matching files to a result
  match: (externalId: number, mediaType: string) => void;
}

export const SearchResultContext = createContext<ISearchResultContext>({
  current: null,
  setCurrent: () => {},
  match: () => {},
});
