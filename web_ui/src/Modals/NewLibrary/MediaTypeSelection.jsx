import React, { useCallback } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import "./MediaTypeSelection.scss";

function MediaTypeSelection(props) {
  const selectMovie = useCallback(() => {
    if (props.mediaType !== "movie") {
      props.setMediaType("movie");
    }
  }, [props.mediaType]);

  const selectTv = useCallback(() => {
    if (props.mediaType !== "tv") {
      props.setMediaType("tv");
    }
  }, [props.mediaType]);

  const selectMixed = useCallback(() => {
    if (props.mediaType !== "mixed") {
      props.setMediaType("mixed");
    }
  }, [props.mediaType]);

  const selectAnime = useCallback(() => {
    if (props.mediaType !== "anime") {
      props.setMediaType("anime");
    }
  }, [props.mediaType]);

  return (
    <div className="mediaTypeSelection">
      <h3>Choose a type</h3>
      <div className="types">
        <div className="type" onClick={selectMovie}>
          <FontAwesomeIcon icon="film"/>
          <p>Movies</p>
          <div className={`select ${props.mediaType === "movie"}`}/>
        </div>
        <div className="type" onClick={selectTv}>
          <FontAwesomeIcon icon="tv"/>
          <p>Shows</p>
          <div className={`select ${props.mediaType === "tv"}`}/>
        </div>
        <div className="type disabled" onClick={selectMixed}>
          <FontAwesomeIcon icon="photo-video"/>
          <p>Mixed</p>
          <div className={`select ${props.mediaType === "mixed"}`}/>
        </div>
        <div className="type disabled" onClick={selectAnime}>
          <FontAwesomeIcon icon="bahai"/>
          <p>Anime</p>
          <div className={`select ${props.mediaType === "anime"}`}/>
        </div>
      </div>
    </div>
  )
};

export default MediaTypeSelection;
