import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import "./ProgressBar.scss";

function ProgressBar(props) {
  const { season, episode } = props.data;
  let { delta, duration } = props.data;

  delta = !delta ? 0 : delta;
  duration = Math.round(duration / 60);

  const current = Math.round(delta / 60);
  const width = current / duration * 100 + "%";

  return (
    <div className="banner-progress-bar">
      {(season && episode) && (
        <div className="s-e">
          <p>S{season}</p>
          <FontAwesomeIcon icon="circle"/>
          <p>E{episode}</p>
        </div>
      )}
      <div className="progress">
        <div className="current">
          <p>{current}</p>
          <p>min</p>
        </div>
        <div className="bar">
          <span className="progress-fill" style={{ width: width }}/>
        </div>
        <div className="duration">
          <p>{duration}</p>
          <p>min</p>
        </div>
      </div>
    </div>
  );
}

export default ProgressBar;
