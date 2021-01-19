import React, { useState, useEffect } from "react";

function CardImage(props) {
  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    const img = new Image();

    img.onload = () => {
      setLoaded(true);
      setErr(false);
    };

    img.onerror = () => {
      setLoaded(true);
      setErr(true);
    };

    img.src = props.src;
  }, [props.src]);

  return (
    <div className="cardImageWrapper">
      {(loaded && !error) && (
        <img src={props.src} alt="cover"/>
      )}
      {loaded && error && (
        <div className="placeholder"/>
      )}
    </div>
  )
}

export default CardImage;
