import Movie from "assets/figma_icons/Movie";
import TV from "assets/figma_icons/TV";

import "./MediaTypeSelector.scss";

export interface SelectMediatypeProps {
  isReady: boolean;
  selectMediatype: (mediatype: string) => void;
}

export const SelectMediatype = ({
  isReady,
  selectMediatype,
}: SelectMediatypeProps) => {
  return (
    <div className={`select-mediatype ready-${isReady}`}>
      <div className="select-label">Select media type</div>
      <div className="select-subtext">Choose media type to search for</div>
      <div className={`select-options ready-${isReady}`}>
        <div className="option" onClick={() => selectMediatype("Movies")}>
          <Movie />
          <p>Movies</p>
        </div>
        <div className="option" onClick={() => selectMediatype("TV Shows")}>
          <TV />
          <p>Shows</p>
        </div>
      </div>
    </div>
  );
};

export default SelectMediatype;
