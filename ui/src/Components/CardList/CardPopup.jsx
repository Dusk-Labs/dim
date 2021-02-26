import React, { useCallback, useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";

import TruncText from "../../Helpers/TruncText.jsx";
import IMDbLogo from "../../assets/imdb.png";

import "./CardPopup.scss";

function CardPopup(props) {
  const [overflowing, setOverflowing] = useState(false);
  const [direction, setDirection] = useState("card-popup-right");

  const onAnimationEnd = useCallback(e => {
    if (e.animationName !== "CardPopupHide") return;
    props.setHovering(false);
  }, []);

  useEffect(() => {
    const { x, width } = props.popup.current.getBoundingClientRect();
    const overflowing = (x + width > window.innerWidth - 5);

    if (!overflowing) return;

    setOverflowing(true);
    setDirection("card-popup-left");
  }, [overflowing]);

  const {
    id,
    name,
    rating,
    description,
    genres,
    year,
    duration,
    seasons
  } = props.data;

  const length = {
    hh: ("0" + Math.floor(duration / 3600)).slice(-2),
    mm: ("0" + Math.floor((duration % 3600) / 60)).slice(-2),
    ss: ("0" + Math.floor((duration % 3600) % 60)).slice(-2)
  };

  if (genres.length > 3) {
    genres.length = 3
  }

  return (
    <div
      className={direction}
      ref={props.popup}
      onAnimationEnd={onAnimationEnd}
    >
      <div className="clipped"/>
      <div className="contentWrapper">
        <section className="header">
          <h2><TruncText content={name} max={8}/></h2>
          <div className="rating">
            <p>{rating || 0}</p>
            <img alt="imdb" src={IMDbLogo}/>
          </div>
        </section>
        <section className="separator"/>
        <section className="description">
          {description !== null && description.length > 0
            ? <p><TruncText content={description} max={21}/></p>
            : <p>No description found.</p>
          }
        </section>
        <section className="tags">
          <Link to={`/search?year=${year}`}>{year}</Link>
          <FontAwesomeIcon icon="circle"/>
          <div className="genres">
            {genres.map((genre, i) => (
              <Link
                to={`/search?genre=${genre}`}
                key={i}
              >
                {genre}
              </Link>
            ))}
          </div>
        </section>
        <section className="separator"/>
        <section className="footer">
          {!seasons && (
            <div className="length">
              <p>{length.hh}:{length.mm}:{length.ss}</p>
              <p>HH MM SS</p>
            </div>
          )}
          {seasons && (
            <div className="length">
              <p>{seasons}</p>
              <p>SEASONS</p>
            </div>
          )}
          <Link to={`/play/${id}`}>
            <p>Play media</p>
            <FontAwesomeIcon icon="play"/>
          </Link>
        </section>
      </div>
    </div>
  );
}

export default CardPopup;
