import { useState, useCallback, useEffect } from "react";
import NestedFileView from "Components/NestedFileView/Index";
import SimpleSearch from "Components/SimpleSearch";
import AdvancedSearch from "Components/AdvancedSearch/Index";
import { SearchResultContext } from "./Context";
import { SearchResults } from "./Results";
import { SelectMediatype } from "./MediaTypeSelector";

import { useMatchMediafilesQuery } from "api/v1/mediafile";
import { UnmatchedMediaFiles } from "api/v1/unmatchedMedia";

import AngleUp from "assets/Icons/AngleUp";
import "./Index.scss";
import { useDispatch } from "react-redux";
import { addNotification } from "slices/notifications";

interface MatchMediaProps {
  data: UnmatchedMediaFiles;
  refetch: () => any;
  mediafileSearch: (query: string) => void;
}

const MatchMedia = ({ data, refetch, mediafileSearch }: MatchMediaProps) => {
  const dispatch = useDispatch();
  const [current, setCurrent] = useState<number | null>(null);
  const [isOpened, setOpened] = useState<boolean>(true);
  const [showSuggestions, setShowSuggestions] = useState<boolean>(false);
  // all the states needed to search external provider
  const [searchQuery, setSearchQuery] = useState<string | null>(null);
  const [searchParams, setSearchParams] = useState<Array<
    Record<string, any>
  > | null>(null);
  // contains list of mediafile ids.
  const [selectedFiles, setSelectedFiles] = useState<number[]>([]);
  const [selectedMediatype, setMediatype] = useState<string | null>(null);
  const [startMatch, setStartMatch] = useState<boolean>(false);
  const [externalId, setExternalId] = useState<number | null>(null);
  const [mediaType, setMediaType] = useState<string | null>(null);
  const [screenSize, setScreenSize] = useState(window.innerWidth);
  const matchResult = useMatchMediafilesQuery(
    {
      ids: selectedFiles,
      external_id: externalId ?? 0,
      media_type: mediaType ?? "",
    },
    { skip: !startMatch }
  );

  // reesize screen size on window resize
  useEffect(() => {
    const handleResize = () => {
      setScreenSize(window.innerWidth);
    };
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  useEffect(() => {
    if (matchResult.isFetching) return;
    if (!startMatch) return;

    if (matchResult.error) {
      console.log("matching error: ", matchResult.error);
      dispatch(
        addNotification({
          msg: "Error matching files.",
        })
      );
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
    startMatch,
    refetch,
    dispatch,
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
      setSearchQuery("");
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
      toggleSuggestionsOff();
    },
    [setSearchQuery, setSearchParams, toggleSuggestionsOff]
  );

  const selectMediatype = useCallback(
    (mediatype) => {
      setMediatype(mediatype);
    },
    [setMediatype]
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

  return (
    <div className={`match-media open-${isOpened}`}>
      <div className="match-container">
        <div className="match-left">
          <p className="match-head">{data.count} Unmatched files found</p>
          {isOpened && (
            <div className="match-middle">
              {screenSize > 1080 && (
                <p className="match-label">View and select files to match.</p>
              )}
              <SimpleSearch onChange={mediafileSearch} />
            </div>
          )}

          <NestedFileView
            files={data.files}
            select={selectFile}
            unselect={unselectFile}
          />
        </div>
        <div className="match-right">
          <div className="right-head">
            {!!selectedMediatype && (
              <AdvancedSearch
                hideSearchBar={!isOpened}
                showSuggestions={showSuggestions}
                toggleSuggestionsOn={toggleSuggestionsOn}
                toggleSuggestionsOff={toggleSuggestionsOff}
                onSearch={onSearch}
                mediatype={selectedMediatype}
                query={searchQuery}
              />
            )}
            <div
              className={`toggle ${!isOpened ? "invert" : ""}`}
              onClick={toggleOpen}
            >
              <AngleUp />
            </div>
          </div>
          <div className="right-content">
            {!selectedMediatype && (
              <SelectMediatype
                isReady={selectedFiles.length > 0}
                selectMediatype={selectMediatype}
              />
            )}
            {!!selectedMediatype && (
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
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default MatchMedia;
