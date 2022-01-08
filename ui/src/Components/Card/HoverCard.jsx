import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link } from "react-router-dom";

import { fetchMediaInfo } from "../../actions/media.js";

import TruncText from "../../Helpers/TruncText";
import IMDbLogo from "../../assets/IMDB";
import CircleIcon from "../../assets/Icons/Circle";
import SelectMediaFile from "../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../Modals/SelectMediaFile/Activators/PlayButton";

import "./HoverCard.scss";

function HoverCard(props) {
  const dispatch = useDispatch();

  const { media } = useSelector((store) => ({
    media: store.media,
  }));

  const { setHovering } = props;

  const onAnimationEnd = useCallback(
    (e) => {
      if (e.animationName !== "CardPopupHide") return;

      setHovering(false);
    },
    [setHovering]
  );

  const { id, name } = props.data;

  useEffect(() => {
    if (!id) return;

    dispatch(fetchMediaInfo(id));
  }, [dispatch, id]);

  if (!media[id]) return null;

  const { info } = media[id];
  const { data, fetched, error } = info;

  // FETCH_MEDIA_INFO_ERR
  if (fetched && error) {
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

  // FETCH_MEDIA_INFO_OK
  if (fetched && !error) {
    const {
      duration,
      genres,
      rating,
      description,
      year,
      progress,
      season,
      episode,
      play_btn_id,
    } = data;

    const length = {
      hh: ("0" + Math.floor(duration / 3600)).slice(-2),
      mm: ("0" + Math.floor((duration % 3600) / 60)).slice(-2),
      ss: ("0" + Math.floor((duration % 3600) % 60)).slice(-2),
    };

    if (genres.length > 3) {
      genres.length = 3;
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
          {year && genres && (
            <section className="tags">
              <Link to={`/search?year=${year}`}>{year}</Link>
              {genres.length > 0 && <CircleIcon />}
              <div className="genres">
                {genres.map((genre, i) => (
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
