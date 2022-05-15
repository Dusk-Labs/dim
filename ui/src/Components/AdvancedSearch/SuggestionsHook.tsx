import { useCallback, useState } from "react";

export interface SuggestionHint {
  name: string;
  description: string;
  // If no filter is attached, assume this is a value for a tag.
  filter?: (input: string) => any;
  // Possible options for this tag
  options?: Array<SuggestionHint>;
  isHidden?: boolean;
  enableIf?: (selected?: string) => boolean;
}

export interface useSuggestionsState {
  suggestions: Array<SuggestionHint>;
  selected?: string;
  clearSelected: () => void;
  advanceTree: (tag: string) => void;
  resetTree: () => void;
  getFilterFn: (tag: string) => ((value: string) => any) | undefined;
  selectByName: (tag_name: string) => void;
}

export const useSuggestions = (initial: Array<SuggestionHint> | null) => {
  // Basically I think theres no need to overcomplicate the way we select and store suggestions
  // So I think it might be a good idea to store all suggestions as one 1d array and have
  // sub-suggestions/options have a `enableIf` method that will return whether this option should be
  // showable or just hidden away.
  const [suggestionsTree, setSuggestionsTree] = useState<Array<SuggestionHint>>(
    initial ? initial : []
  );
  const [suggestions, setSuggestions] = useState<Array<SuggestionHint>>(
    initial ? initial : []
  );
  const [selected, setSelected] = useState<string | undefined>(undefined);

  // attempts to advance the suggestions tree forward
  const advanceTree = useCallback(
    (tag: string) => {
      const options = suggestionsTree.find((x) => x.name === tag)?.options;

      if (!options) return;

      // if tree depth is more than 1 this will break the tree
      // this is necessary so that we persist isHidden
      setSuggestionsTree(suggestions);
      setSuggestions(options);
    },
    [setSuggestions, suggestionsTree, setSuggestionsTree, suggestions]
  );

  const resetTree = useCallback(() => {
    setSuggestions(suggestionsTree);
  }, [setSuggestions, suggestionsTree]);

  const selectByName = useCallback(
    (tag_name) => {
      setSelected(tag_name);
    },
    [setSelected]
  );

  const getFilterFn = useCallback(
    (tag: string) => {
      const filter = suggestionsTree.find((x) => x.name === tag)?.filter;
      if (!filter) return undefined;

      return filter;
    },
    [suggestionsTree]
  );

  const clearSelected = useCallback(() => {
    setSelected(undefined);
  }, [setSelected]);

  return {
    suggestions,
    selected,
    clearSelected,
    advanceTree,
    resetTree,
    getFilterFn,
    selectByName,
  };
};
