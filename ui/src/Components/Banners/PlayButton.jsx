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
    >
      <p style={accentCSS}>Play media</p>
      <FontAwesomeIcon icon="play"/>
    </Link>
  );
}

export default PlayButton;
