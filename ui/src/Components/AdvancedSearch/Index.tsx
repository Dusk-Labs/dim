import { useState, useCallback, useRef } from "react";
import SearchIcon from "assets/figma_icons/Search";
import Suggestions from "./Suggestions";
import { ISearchTag, SearchTag, useSearchTags } from "./TagHook";
import { useSuggestions } from "./SuggestionsHook";
import { SearchContext } from "./Context";
import "./Index.scss";

const filterInt = (input: string) => {
  const parsed = Number(input);
  return isNaN(parsed) ? null : parsed;
};

const isOption = (options: string[]) => {
  return (input: string) => {
    return options.includes(input) ? input : null;
  };
};

type Props = {
  hideSearchBar: boolean;
  showSuggestions: boolean;
  toggleSuggestionsOn: () => void;
  toggleSuggestionsOff: () => void;
  onSearch: (query: string, params: Array<ISearchTag>) => void;
};

export const AdvancedSearch = (props: Props) => {
  const { hideSearchBar, showSuggestions, toggleSuggestionsOn, onSearch } =
    props;

  const [value, setValue] = useState<string>("");
  const inputRef = useRef<HTMLDivElement>(null);
  const { activeTags, appendTag, setTagValue, popTag, lastTag } =
    useSearchTags();

  const mediaHints = [
    { name: "Movies", description: "Search for Movies." },
    { name: "TV Shows", description: "Search for TV Shows." },
  ];

  const suggestionsState = useSuggestions([
    {
      name: "Year",
      description: "filter search results by year.",
      filter: filterInt,
      options: [],
    },
    {
      name: "Media",
      description: "filter search results by media type (Movies, or TV Shows).",
      filter: isOption(["Movies", "TV Shows"]),
      options: mediaHints,
    },
  ]);

  const {
    suggestions,
    selectNext,
    selectPrev,
    selected,
    clearSelected,
    advanceTree,
    resetTree,
    getFilterFn,
  } = suggestionsState;

  const onInput = useCallback(
    (e) => {
      setValue(e.target.innerText);
    },
    [setValue]
  );

  // Callback attempt to parse input and append it to a un-filled tag if possible.
  const matchInput = useCallback(
    (e) => {
      if (activeTags.length === 0) return;

      const lastTag = activeTags[activeTags.length - 1];

      if (!lastTag || typeof suggestions === "undefined") return;

      const caretPosition = document?.getSelection()?.focusOffset;
      const focusedValue = value.substring(0, caretPosition);

      // @ts-ignore
      const suggestionsFilter = getFilterFn(lastTag!.name);

      if (suggestionsFilter) {
        const filteredValue = suggestionsFilter(focusedValue);
        if (filteredValue) {
          setTagValue(lastTag.name, filteredValue);

          // clear the parsed value from the main input
          const rest = value.substring(caretPosition || 0, value.length);
          inputRef.current!.innerText = rest;

          // If this event was triggered by a space key, we want to prevent the event
          // so that it doesnt show up in our search bar.
          if (e) e.preventDefault();

          resetTree();
        }
      }
    },
    [
      value,
      activeTags,
      suggestions,
      inputRef,
      setTagValue,
      resetTree,
      getFilterFn,
    ]
  );

  const onSuggestionClick = useCallback(
    (tag: string) => {
      // Clear the currently highlighted suggestion.
      clearSelected();
      toggleSuggestionsOn();
      // focus back on the input so that we can continue typing.
      // if theres an active tag that has no value set, we want to set its value given the click of this suggestion.
      const last = lastTag();
      if (last && last.content === "") {
        setTagValue(last.name, tag);
        resetTree();
        return;
      }

      if (inputRef?.current) inputRef.current!.focus();
      appendTag({ name: tag, content: "" });
      advanceTree(tag);
    },
    [
      appendTag,
      advanceTree,
      inputRef,
      lastTag,
      setTagValue,
      resetTree,
      clearSelected,
      toggleSuggestionsOn,
    ]
  );

  // called to prepare the query and params for the higher-level search functions
  const search = useCallback(() => {
    if (!value) return;

    onSearch(value, activeTags);
  }, [value, activeTags, onSearch]);

  const onEnter = useCallback(() => {
    console.log(selected);
    if (!selected) {
      if (value) search();
      return;
    }

    if (!suggestions.find((x) => x.name === selected)) return;
    if (activeTags.find((x) => x.name === selected)) return;

    onSuggestionClick(selected);
  }, [selected, onSuggestionClick, suggestions, activeTags, value, search]);

  const onBackspace = useCallback(() => {
    const caretPosition = document?.getSelection()?.focusOffset;

    // only pop the last tag if we run backspace on the tag
    if (caretPosition === 0) {
      const tag = popTag();
      if (tag) {
        resetTree();
      }
    }
  }, [popTag, resetTree]);

  const onKeyDown = useCallback(
    (e) => {
      if (e.key === "ArrowDown") selectNext();

      if (e.key === "ArrowUp") selectPrev();

      if (e.key === "Enter") {
        // Prevent deault event handler so we dont get newlines in our div.
        e.preventDefault();
        onEnter();
      }

      // Some tags such as year might be filled after a space automatically
      if (e.key === " ") {
        matchInput(e);
      }

      if (e.key === "Backspace") {
        onBackspace();
      }
    },
    [selectNext, matchInput, selectPrev, onEnter, onBackspace]
  );

  return (
    <>
      <div className={`advanced-search hidden-${hideSearchBar}`}>
        <div className="advanced-search-wrapper">
          <div className="advanced-search-field">
            {activeTags.map(({ name, content }) => (
              <SearchTag name={name} content={content} />
            ))}
            <div
              className="advanced-search-input"
              onKeyDown={onKeyDown}
              onInput={onInput}
              onFocus={toggleSuggestionsOn}
              contentEditable="true"
              ref={inputRef}
              spellCheck="false"
              placeholder="Search..."
            />
          </div>
          <SearchIcon />
        </div>
      </div>
      <SearchContext.Provider
        value={{ active: showSuggestions, suggestionsState, activeTags }}
      >
        <Suggestions onClick={onSuggestionClick} />
      </SearchContext.Provider>
    </>
  );
};

export default AdvancedSearch;
