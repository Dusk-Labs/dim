import { useState, useCallback, useEffect } from "react";
import { useParams } from "react-router-dom";
import NestedFileView from "Components/NestedFileView/Index";
import SimpleSearch from "Components/SimpleSearch";
import AdvancedSearch from "Components/AdvancedSearch/Index";
import { SearchResultContext } from "./Context";
import { SearchResults } from "./Results";

import { useGetUnmatchedMediaFilesQuery } from "api/v1/unmatchedMedia";
import { useMatchMediafilesQuery } from "api/v1/mediafile";

import AngleUp from "assets/Icons/AngleUp";
import "./Index.scss";

interface LibraryParams {
  id?: string | undefined;
}

const MatchMedia = () => {
  const { id } = useParams<LibraryParams>();
  const [current, setCurrent] = useState<number | null>(null);
  const [isOpened, setOpened] = useState<boolean>(true);
  const [showSuggestions, setShowSuggestions] = useState<boolean>(false);
  const { data, refetch } = useGetUnmatchedMediaFilesQuery(id!);
  // all the states needed to search external provider
  const [searchQuery, setSearchQuery] = useState<string | null>(null);
  const [searchParams, setSearchParams] = useState<Array<
    Record<string, any>
  > | null>(null);
  // contains list of mediafile ids.
  const [selectedFiles, setSelectedFiles] = useState<number[]>([]);

  const [startMatch, setStartMatch] = useState<boolean>(false);
  const [externalId, setExternalId] = useState<number | null>(null);
  const [mediaType, setMediaType] = useState<string | null>(null);
  const matchResult = useMatchMediafilesQuery(
    {
      ids: selectedFiles,
      external_id: externalId ?? 0,
      media_type: mediaType ?? "",
    },
    { skip: !startMatch }
  );

  useEffect(() => {
    if (matchResult.isFetching) return;

    if (matchResult.error) {
      console.log("matching error: ", matchResult.error);
      return;
    }

    if (!matchResult.error) {
      setStartMatch(false);
      setSelectedFiles([]);
    }

    console.log("matched");
    refetch();
  }, [
    matchResult.data,
    matchResult.error,
    matchResult.isFetching,
    setSelectedFiles,
    refetch,
  ]);

  const setCurrentCallback = useCallback(
    (current: number | null) => {
      setCurrent(current);
    },
    [setCurrent]
  );

  const toggleOpen = useCallback(() => {
    setOpened(!isOpened);
  }, [isOpened, setOpened]);

  const toggleSuggestionsOn = useCallback(() => {
    setShowSuggestions(true);
  }, [setShowSuggestions]);

  const toggleSuggestionsOff = useCallback(() => {
    setShowSuggestions(false);
  }, [setShowSuggestions]);

  const matchSelected = useCallback(
    (externalId: number, media_type: string) => {
      console.log("Matching selected files.");
      setExternalId(externalId);
      setMediaType(media_type);
      setStartMatch(true);
    },
    [setStartMatch, setExternalId, setMediaType]
  );

  const selectFile = useCallback(
    (id: number) => {
      if (id in selectedFiles) return;
      setSelectedFiles([...selectedFiles, id]);
    },
    [selectedFiles, setSelectedFiles]
  );

  const unselectFile = useCallback(
    (id: number) => {
      setSelectedFiles(selectedFiles.filter((x: number) => x !== id));
    },
    [selectedFiles, setSelectedFiles]
  );

  const onSearch = useCallback(
    (query, params) => {
      if (!query || query === "") return;

      setSearchQuery(query);
      setSearchParams(params);
    },
    [setSearchQuery, setSearchParams]
  );

  // effect needed so that we can hide suggestions when the user clicks outside the container.
  useEffect(() => {
    const outsideClickListener = (event: any) => {
      if (!event.target.closest(".advanced-search, .suggestions")) {
        toggleSuggestionsOff();
      }
    };

    document.addEventListener("click", outsideClickListener);

    return () => {
      document.removeEventListener("click", outsideClickListener);
    };
  }, [toggleSuggestionsOff]);

  const files = !data
    ? []
    : Object.entries(data).map(([key, value]) => {
        return {
          name: key,
          type: "folder",
          content: value.map((file) => {
            return {
              name: file.name,
              id: file.id,
              type: "file",
            };
          }),
        };
      });

  // TODO: Display errors if any.
  return (
    <div className={`match-media open-${isOpened}`}>
      <div className="match-container">
        <div className="match-left">
          <p className="match-head">3 Unmatched files found</p>
          <div className="match-middle">
            <p className="match-label">View and select files to match.</p>
            <SimpleSearch />
          </div>

          <NestedFileView
            files={files}
            select={selectFile}
            unselect={unselectFile}
          />
        </div>
        <div className="match-right">
          <div className="search-head">
            <AdvancedSearch
              hideSearchBar={!isOpened}
              showSuggestions={showSuggestions}
              toggleSuggestionsOn={toggleSuggestionsOn}
              toggleSuggestionsOff={toggleSuggestionsOff}
              onSearch={onSearch}
            />
            <div
              className={`toggle ${!isOpened ? "invert" : ""}`}
              onClick={toggleOpen}
            >
              <AngleUp />
            </div>
          </div>

          <div className="search-results">
            <SearchResultContext.Provider
              value={{
                current,
                setCurrent: setCurrentCallback,
                match: matchSelected,
              }}
            >
              {searchQuery && searchParams ? (
                <SearchResults query={searchQuery} params={searchParams} />
              ) : null}
            </SearchResultContext.Provider>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MatchMedia;
