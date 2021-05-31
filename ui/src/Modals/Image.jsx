import { useState, useEffect } from "react";

import DimLogo from "../assets/DimLogo";

function CardImage(props) {
  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);
  const [imageSrc, setImageSrc] = useState();

  useEffect(() => {
    if (!props.src) {
      setLoaded(true);
      setErr(true);
      return;
    }

    const img = new Image();

    img.onload = () => {
      console.log("[Image] loaded");
      setLoaded(true);
      setImageSrc(img.src);
    };

    img.onerror = (e) => {
      setLoaded(true);
      setErr(true);

      img.src = "";
      img.src = props.src;

      console.log("[Image] err", e);
    };

    img.src = props.src;
  }, [props.src]);

  return (
    <div className="cardImageWrapper">
      {(loaded && !error) && (
        <img src={imageSrc} alt="cover"/>
      )}
      {error && (
        <div className="placeholder">
          <DimLogo/>
        </div>
      )}
    </div>
  );
}

export default CardImage;
