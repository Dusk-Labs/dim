import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";

import "./PlayButton.scss";

function PlayButton(props) {
  const accentCSS = {
    background: props.bgColor,
    color: props.textColor
  };

  return (
    <Link
      to={`/play/${props.id}`}
      className="bannerPlayBtn"
      style={accentCSS}
    >
      PLAY
      <FontAwesomeIcon icon="play-circle"/>
    </Link>
  );
}

export default PlayButton;
