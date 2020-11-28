import React, { useState } from "react";
import { Link } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import ProgressBar from "./ProgressBar.jsx";
import Image from "./Image.jsx";
import PlayButton from "./PlayButton.jsx";
import TruncText from "../../Helpers/TruncText.jsx";

import "./Banner.scss";

function Banner(props) {
  const [backgroundColor, setBackgroundColor] = useState("#f7931e");
  const [textColor, setTextColor] = useState("#fff");

  if (!props.data) {
    return (
      <div className="banner">
        <div className="placeholder">
          <p>Empty</p>
        </div>
      </div>
    );
  }

  const {
    id,
    title,
    year,
    synopsis,
    backdrop,
    delta,
    duration,
    genres,
    season,
    episode
  } = props.data;

  if (genres.length > 3) {
    genres.length = 3;
  }

  const progressBarData = {
    backgroundColor,
    season,
    episode,
    duration,
    delta
  };

  return (
    <div className="banner">
      <Image
        setText={setTextColor}
        setBG={setBackgroundColor}
        src={backdrop}
        hideAnimationName="onHideBannerImage"
      />
      <div className="extras">
        <Link to={`/search?year=${year}`}>{year}</Link>
        <FontAwesomeIcon icon="circle" style={{ color: backgroundColor }}/>
        {genres.map((genre, i) => (
          <Link
            to={`/search?genre=${genre}`}
            key={i}
          >
            {genre}
          </Link>
        ))}
      </div>
      <div className="info">
        <h1>{title}</h1>
        <p className="description">
          <TruncText content={synopsis} max={35}/>
        </p>
        <PlayButton
          id={id}
          bgColor={backgroundColor}
          textColor={textColor}
        />
      </div>
      <ProgressBar data={progressBarData}/>
    </div>
  );
}

export default Banner;
