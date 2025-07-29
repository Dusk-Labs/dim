import { useState, useCallback, useEffect } from "react";
import SearchIcon from "assets/figma_icons/Search";
import "./SimpleSearch.scss";

export interface SimpleSearchProps {
  placeholder?: string;
  onChange?: (query: string) => void;
}

export const SimpleSearch = ({ placeholder, onChange }: SimpleSearchProps) => {
  const [value, setValue] = useState<string>("");

  useEffect(() => {
    if (onChange) onChange(value);
  }, [value, onChange]);

  const changeValue = useCallback(
    (e) => {
      setValue(e?.target?.value || "");
    },
    [setValue]
  );

  return (
    <div className="simple-searchbox">
      <SearchIcon />
      <input
        type="text"
        placeholder={placeholder ? placeholder : "Search files to match"}
        onChange={changeValue}
      />
    </div>
  );
};

export default SimpleSearch;
