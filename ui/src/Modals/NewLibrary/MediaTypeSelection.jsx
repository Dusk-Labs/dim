import { useCallback } from "react";

import FilmIcon from "../../assets/Icons/Film";
import TvIcon from "../../assets/Icons/TvIcon";
import PhotoVideoIcon from "../../assets/Icons/PhotoVideo";
import AnimeIcon from "../../assets/Icons/Anime";

import "./MediaTypeSelection.scss";

function MediaTypeSelection(props) {
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

  const selectMixed = useCallback(() => {
    if (mediaType !== "mixed") {
      setMediaType("mixed");
    }
  }, [mediaType, setMediaType]);

  const selectAnime = useCallback(() => {
    if (mediaType !== "anime") {
      setMediaType("anime");
    }
  }, [mediaType, setMediaType]);

  return (
    <div className="mediaTypeSelection">
      <h3>Choose a type</h3>
      <div className="types">
        <div className="type" onClick={selectMovie}>
          <FilmIcon/>
          <p>Movies</p>
          <div className={`select ${props.mediaType === "movie"}`}/>
        </div>
        <div className="type" onClick={selectTv}>
          <TvIcon/>
          <p>Shows</p>
          <div className={`select ${props.mediaType === "tv"}`}/>
        </div>
        <div className="type disabled" onClick={selectMixed}>
          <PhotoVideoIcon/>
          <p>Mixed</p>
          <div className={`select ${props.mediaType === "mixed"}`}/>
        </div>
        <div className="type disabled" onClick={selectAnime}>
          <AnimeIcon/>
          <p>Anime</p>
          <div className={`select ${props.mediaType === "anime"}`}/>
        </div>
      </div>
    </div>
  );
}

export default MediaTypeSelection;
