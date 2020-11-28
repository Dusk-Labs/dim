import React, { useCallback, useRef, useState } from "react";
import { Link } from "react-router-dom";

import CardPopup from "./CardPopup.jsx";
import Image from "./Image";

import "./Card.scss";

function Card(props) {
  const cardWrapper = useRef(null);
  const cardPopup = useRef(null);

  const [hovering, setHovering] = useState(false);
  const [timeoutID, setTimeoutID] = useState(null);

  const [backgroundColor, setBackgroundColor] = useState("f7931e");
  const [textColor, setTextColor] = useState("#fff");

  const showPopup = useCallback(async () => {
    setHovering(true);
  }, []);

  const onMouseLeave = useCallback(() => {
    clearTimeout(timeoutID);

    cardPopup.current?.classList.add("hideCardPopup");
  }, [timeoutID, cardPopup]);

  const handleMouseEnter = useCallback(() => {
    if (hovering || window.innerWidth < 1300) return;

    const ID = setTimeout(showPopup, 600);
    setTimeoutID(ID);
  }, [hovering]);

  const { name, poster_path, id } = props.data;

  const accent = {
    background: backgroundColor,
    text: textColor
  };

  const data = {
    ...props.data,
    accent
  };

  return (
    <div
      className="card-wrapper"
      ref={cardWrapper}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      <div id={id} className="card">
        <Link to={`/media/${id}`}>
          <Image
            src={poster_path}
            setBG={setBackgroundColor}
            setText={setTextColor}
          />
          <p style={{opacity: + !hovering}}>{name}</p>
        </Link>
      </div>
      {hovering && (
        <CardPopup
          popup={cardPopup}
          data={data}
          setHovering={setHovering}
        />
      )}
    </div>
  );
}

export default Card;