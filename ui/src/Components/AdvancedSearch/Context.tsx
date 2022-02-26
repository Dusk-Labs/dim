import { createContext } from "react";
import { useSuggestionsState } from "./SuggestionsHook";
import { ISearchTag } from "./TagHook";

interface ISearchContext {
  active: boolean;
  suggestionsState: useSuggestionsState;
  activeTags: Array<ISearchTag>;
}

export const SearchContext = createContext<ISearchContext>({
  active: false,
  suggestionsState: {
    suggestions: [],
    selectNext: () => {},
    selectPrev: () => {},
    selected: undefined,
    clearSelected: () => {},
    advanceTree: (_) => {},
    resetTree: () => {},
    getFilterFn: (_) => {
      return undefined;
    },
  },
  activeTags: [],
});
