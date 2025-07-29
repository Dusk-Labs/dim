import { createContext } from "react";
import { useSuggestionsState } from "./SuggestionsHook";
import { ISearchTag } from "./TagHook";

interface ISearchContext {
  active: boolean;
  suggestionsState: useSuggestionsState;
  activeTags: Array<ISearchTag>;
  input: string;
}

export const SearchContext = createContext<ISearchContext>({
  active: false,
  suggestionsState: {
    suggestions: [],
    selected: undefined,
    clearSelected: () => {},
    advanceTree: (_) => {},
    resetTree: () => {},
    getFilterFn: (_) => {
      return undefined;
    },
    selectByName: (_) => {},
  },
  activeTags: [],
  input: "",
});
