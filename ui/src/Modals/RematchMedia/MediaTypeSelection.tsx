import { useCallback } from "react";

import FilmIcon from "assets/Icons/Film";
import TvIcon from "assets/Icons/TvIcon";

import "./MediaTypeSelection.scss";

interface Props {
  mediaType: string;
  setMediaType: React.Dispatch<React.SetStateAction<string>>;
}

function MediaTypeSelection(props: Props) {
  const { mediaType, setMediaType } = props;

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
    <div className="mediaTypeSelection">
      <h4>Choose a type</h4>
      <div className="types">
        <div className="type" onClick={selectMovie}>
          <FilmIcon />
          <p>Movies</p>
          <div className={`select ${props.mediaType === "movie"}`} />
        </div>
        <div className="type" onClick={selectTv}>
          <TvIcon />
          <p>Shows</p>
          <div className={`select ${props.mediaType === "tv"}`} />
        </div>
      </div>
    </div>
  );
}

export default MediaTypeSelection;
