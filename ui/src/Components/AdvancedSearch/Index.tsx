import { useState, useCallback } from "react";
import SearchIcon from "assets/figma_icons/Search";
import "./Index.scss";

export const SearchTag = () => {
  return (
    <span className="advanced-search-tag">
      <p>Media: TV Show</p>
    </span>
  );
};

export const AdvancedSearch = () => {
  const [value, setValue] = useState<string>("");
  const onInput = useCallback(
    (e) => {
      setValue(e.target.innerText);
    },
    [setValue]
  );

  console.log(value);

  return (
    <div className="advanced-search-outer">
      <div className="advanced-search">
        <SearchTag />
        <div
          className="advanced-search-input"
          onInput={onInput}
          contentEditable="true"
          spellCheck="false"
        />
      </div>
      <SearchIcon />
    </div>
  );
};

export default AdvancedSearch;
