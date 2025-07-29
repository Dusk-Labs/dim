import { useCallback } from "react";
import { skipToken } from "@reduxjs/toolkit/query/react";
import { Link } from "react-router-dom";

import { useGetMediaQuery } from "../../api/v1/media";

import TruncText from "../../Helpers/TruncText";
import IMDbLogo from "../../assets/IMDB";
import CircleIcon from "../../assets/Icons/Circle";
import SelectMediaFile from "../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../Modals/SelectMediaFile/Activators/PlayButton";

import "./HoverCard.scss";

function HoverCard(props) {
  const { setHovering } = props;

  const onAnimationEnd = useCallback(
    (e) => {
      if (e.animationName !== "CardPopupHide") return;

      setHovering(false);
    },
    [setHovering]
  );

  const { id, name } = props.data;

  const { data, isError } = useGetMediaQuery(id ? id : skipToken);

  if (isError) {
    return (
      <div
        className={
          props.side === "right" ? "card-popup-right" : "card-popup-left"
        }
        ref={props.popup}
        onAnimationEnd={onAnimationEnd}
      >
        <div className="clipped" />
        <div className="contentWrapper">
          <section className="header">
            <h2>Failed to load media</h2>
          </section>
          <section className="separator" />
          <section className="description">
            <p>Something went wrong somewhere.</p>
          </section>
        </div>
      </div>
    );
  }

  if (data) {
    const {
      duration,
      rating,
      description,
      year,
      progress,
      season,
      episode,
      play_btn_id,
      genres,
    } = data;

    // copy needed so that `Array.splice` doesnt complain.
    const genresCopy = [...genres];

    const length = {
      hh: ("0" + Math.floor(duration / 3600)).slice(-2),
      mm: ("0" + Math.floor((duration % 3600) / 60)).slice(-2),
      ss: ("0" + Math.floor((duration % 3600) % 60)).slice(-2),
    };

    if (genresCopy.length > 3) {
      genresCopy.splice(3);
    }

    return (
      <div
        className={
          props.side === "right" ? "card-popup-right" : "card-popup-left"
        }
        ref={props.popup}
        onAnimationEnd={onAnimationEnd}
      >
        <div className="clipped" />
        <div className="contentWrapper">
          <section className="hoverCardHeader">
            <div className="titleWrapper">
              <h2>{name}</h2>
            </div>
            {(rating || rating === 0) && (
              <div className="rating">
                <p>{rating}</p>
                <IMDbLogo />
              </div>
            )}
          </section>
          <section className="separator" />
          <section className="description">
            {description !== null && description.length > 0 ? (
              <p>
                <TruncText content={description} max={21} />
              </p>
            ) : (
              <p>No description found</p>
            )}
          </section>
          {year && genresCopy && (
            <section className="tags">
              <Link to={`/search?year=${year}`}>{year}</Link>
              {genresCopy.length > 0 && <CircleIcon />}
              <div className="genres">
                {genresCopy.map((genre, i) => (
                  <Link
                    to={`/search?genre=${encodeURIComponent(genre)}`}
                    key={i}
                  >
                    {genre}
                  </Link>
                ))}
              </div>
            </section>
          )}
          <section className="separator" />
          <section className="footer">
            <div className="length">
              <p>
                {length.hh}:{length.mm}:{length.ss}
              </p>
              <p>HH MM SS</p>
            </div>
            <SelectMediaFile title={name} mediaID={play_btn_id || id}>
              <SelectMediaFilePlayButton
                progress={progress}
                seasonep={{ season, episode }}
              />
            </SelectMediaFile>
          </section>
        </div>
      </div>
    );
  }

  return null;
}

export default HoverCard;
