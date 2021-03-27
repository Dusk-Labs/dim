import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import SelectMediaVersion from "../Modals/SelectMediaVersion";
import { Link } from "react-router-dom";

import "./PlayButton.scss";

function PlayButton(props) {
  const accentCSS = {
    background: props.bgColor,
    color: props.textColor
  };

  const { versions } = props;

  if (versions.length === 1) {
    return (
      <div>
        <Link to={`/play/${versions[0].id}`} className="playBtn">
          <p style={accentCSS}>Play media</p>
          <FontAwesomeIcon icon="play"/>
        </Link>
      </div>
    )
  } else {
    return (
      <SelectMediaVersion versions={versions}>
        <button className="playBtn">
          <p style={accentCSS}>Play media</p>
          <FontAwesomeIcon icon="play"/>
        </button>
      </SelectMediaVersion>
    );
  }
}

export default PlayButton;
