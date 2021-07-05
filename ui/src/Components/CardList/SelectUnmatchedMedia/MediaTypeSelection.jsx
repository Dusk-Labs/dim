import { useCallback, useContext } from "react";

import FilmIcon from "../../../assets/Icons/Film";
import TvIcon from "../../../assets/Icons/TvIcon";
import { SelectUnmatchedContext } from "./Context";

import "./MediaTypeSelection.scss";

const SelectUnmatchedMediaTypeSelection = () => {
  const { mediaType, setMediaType } = useContext(SelectUnmatchedContext);

  const selectMovie = useCallback(() => {
    if (mediaType !== "movie") {
      setMediaType("movie");
    }
  }, [mediaType, setMediaType]);

  const selectTv = useCallback(() => {
    if (mediaType !== "tv") {
      setMediaType("tv");
    }
  }, [mediaType, setMediaType]);

  return (
    <div className="selectUnmatchedMediaTypeSelection">
      <div className="types">
        <div className="type" onClick={selectMovie}>
          <FilmIcon/>
          <p>Movie</p>
          <div className={`select ${mediaType === "movie"}`}/>
        </div>
        <div className="type" onClick={selectTv}>
          <TvIcon/>
          <p>Show</p>
          <div className={`select ${mediaType === "tv"}`}/>
        </div>
      </div>
    </div>
  );
};

export default SelectUnmatchedMediaTypeSelection;
