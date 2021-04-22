import { useCallback, useEffect, useRef, useState } from "react";
import { Link } from "react-router-dom";

import CardPopup from "./CardPopup.jsx";
import Image from "./Image";

import "./Card.scss";

function Card(props) {
  const cardWrapper = useRef(null);
  const cardPopup = useRef(null);
  const card = useRef(null);

  const [mediaProgress, setMediaProgress] = useState(0);
  const [hovering, setHovering] = useState(false);
  const [timeoutID, setTimeoutID] = useState(null);

  useEffect(() => {
    return () => {
      clearTimeout(timeoutID);
    }
  }, [timeoutID]);

  const showPopup = useCallback(() => {
    setHovering(true);
  }, []);

  const onMouseLeave = useCallback(() => {
    clearTimeout(timeoutID);

    cardPopup.current?.classList.add("hideCardPopup");
  }, [timeoutID, cardPopup]);

  const handleMouseEnter = useCallback(() => {
    // removes cardHighlight animation (when searched for)
    if (card.current && card.current.style.animation) {
      card.current.style.animation = "";
    }

    if (hovering || window.innerWidth < 1300) return;

    const ID = setTimeout(showPopup, 600);
    setTimeoutID(ID);
  }, [hovering, showPopup]);

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
          <Image src={poster_path} progress={mediaProgress}/>
          <p style={{opacity: + !hovering}}>{name}</p>
        </Link>
      </div>
      {hovering && (
        <CardPopup
          popup={cardPopup}
          data={props.data}
          setHovering={setHovering}
        />
      )}
    </div>
  );
}

export default Card;