import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import SelectMediaVersion from "../../Modals/SelectMediaVersion";

import "./PlayButton.scss";
import { Link } from "react-router-dom";

function PlayButton(props) {
  const accentCSS = {
    background: props.bgColor,
    color: props.textColor
  };

  const { versions } = props;

  if (versions.length === 1) {
    return (
      <Link to={`/play/${versions[0].id}`} className="bannerPlayBtn">
        <p style={accentCSS}>Play media</p>
        <FontAwesomeIcon icon="play"/>
      </Link>
    )
  } else {
    return (
      <SelectMediaVersion versions={versions}>
        <button className="bannerPlayBtn">
          <p style={accentCSS}>Play media</p>
          <FontAwesomeIcon icon="play"/>
        </button>
      </SelectMediaVersion>
    );
  }
}

export default PlayButton;
