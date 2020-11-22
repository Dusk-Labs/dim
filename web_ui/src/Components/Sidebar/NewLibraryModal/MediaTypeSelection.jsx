import React, { useCallback } from "react";

import Icon from "../Icon";
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

  return (
    <div className="mediaTypeSelection">
      <h3>Choose a type</h3>
      <div className="types">
        <div className="type" onClick={selectMovie}>
          <Icon icon="movie"/>
          <p>Movies</p>
          <div className={`select ${props.mediaType === "movie"}`}/>
        </div>
        <div className="type" onClick={selectTv}>
          <Icon icon="tv"/>
          <p>TV Shows</p>
          <div className={`select ${props.mediaType === "tv"}`}/>
        </div>
      </div>
    </div>
  )
};

export default MediaTypeSelection;
