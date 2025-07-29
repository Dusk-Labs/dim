import { createRef, useCallback, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { Link } from "react-router-dom";

import HoverCard from "./HoverCard";
import Image from "./Image";

import "./Index.scss";

function Card(props) {
  const { settings } = useSelector((store) => ({
    settings: store.settings.userSettings,
  }));

  const cardWrapper = useRef(null);
  const hoverCard = createRef();
  const card = useRef(null);

  const [mediaProgress, setMediaProgress] = useState(0);
  const [hovering, setHovering] = useState(false);
  const [timeoutID, setTimeoutID] = useState(null);
  const [hoverCardSide, setHoverCardSide] = useState("right");

  useEffect(() => {
    return () => {
      clearTimeout(timeoutID);
    };
  }, [timeoutID]);

  const showPopup = useCallback(() => {
    setHovering(true);
  }, []);

  const onMouseLeave = useCallback(() => {
    clearTimeout(timeoutID);

    hoverCard.current?.classList.add("hideCardPopup");
  }, [timeoutID, hoverCard]);

  const handleMouseEnter = useCallback(() => {
    // removes cardHighlight animation (when searched for)
    if (card.current && card.current.style.animation) {
      card.current.style.animation = "";
    }

    if (hovering || window.innerWidth < 1400 || !settings.data.show_hovercards)
      return;

    const rect = card.current.getBoundingClientRect();

    const hoverCardWidth = parseInt(
      getComputedStyle(document.documentElement).getPropertyValue(
        "--hoverCardWidth"
      )
    );

    const showHoverOnRight = window.innerWidth - rect.right > hoverCardWidth;
    const side = showHoverOnRight ? "right" : "left";

    setHoverCardSide(side);

    const ID = setTimeout(showPopup, 600);
    setTimeoutID(ID);
  }, [hovering, settings.data.show_hovercards, showPopup]);

  const { name, poster_path, id, media_type } = props.data;

  useEffect(() => {
    if (media_type === "movie") {
      const { duration, progress } = props.data;
      setMediaProgress((progress / duration) * 100);
    }
  }, [media_type, props.data]);

  return (
    <div
      className="card-wrapper"
      ref={cardWrapper}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      <div id={id} className="card" ref={card}>
        <Link to={`/media/${id}`}>
          <Image src={poster_path} progress={mediaProgress} />
          {settings.data.show_card_names && (
            <p style={{ opacity: +!hovering }}>{name}</p>
          )}
        </Link>
      </div>
      {hovering && (
        <HoverCard
          side={hoverCardSide}
          popup={hoverCard}
          data={props.data}
          setHovering={setHovering}
        />
      )}
    </div>
  );
}

export default Card;
