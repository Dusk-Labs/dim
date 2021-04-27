import React from "react";
import { Link } from "react-router-dom";

import SelectMediaVersion from "../Modals/SelectMediaVersion";
import PlayIcon from "../assets/Icons/Play";

import "./PlayButton.scss";

function PlayButton(props) {
  const accentCSS = {
    background: props.bgColor,
    color: props.textColor
  };

  const { versions, mediaID, progress } = props;

  if (versions.length === 1) {
    return (
      <div>
        <Link to={`/play/${versions[0].id}`} className="playBtn">
          <p style={accentCSS}>
            {progress > 0 ? "Resume media" : "Play media"}
          </p>
          <PlayIcon/>
        </Link>
      </div>
    );
  } else {
    return (
      <SelectMediaVersion mediaID={mediaID} versions={versions}>
        <button className="playBtn">
          <p style={accentCSS}>
            {progress > 0 ? "Resume media" : "Play media"}
          </p>
          <PlayIcon/>
        </button>
      </SelectMediaVersion>
    );
  }
}

export default PlayButton;
