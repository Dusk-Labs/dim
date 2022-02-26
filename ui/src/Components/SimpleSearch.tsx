import { useState, useCallback } from "react";
import SearchIcon from "assets/figma_icons/Search";
import "./SimpleSearch.scss";

export const SimpleSearch = (props: any) => {
  const { placeholder, onChange } = props;
  const [value, setValue] = useState<string>("");

  const changeValue = useCallback(
    (e) => {
      setValue(e?.target?.value || "");

      if (onChange) onChange(value);
    },
    [value, setValue, onChange]
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
