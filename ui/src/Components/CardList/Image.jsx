import { useState, useEffect } from "react";

import DimLogo from "../../assets/DimLogo";

function CardImage(props) {
  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);
  const [imageSrc, setImageSrc] = useState();

  useEffect(() => {
    const img = new Image();

    img.onload = () => {
      setLoaded(true);
    };

    img.onerror = () => {
      setLoaded(true);
      setErr(true);
    };

    // test whether the src passed is an absolute URL or not
    // NOTE: see issue #13
    img.src = new RegExp("/^(?:/|[a-z]+://)/").test(props.src)
      ? props.src : `//${window.host}:${window.backend_port}/${props.src}`;

    setImageSrc(img.src);

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
      {props.progress !== undefined && (
        <div className="progress">
          <div className="value" style={{width: `${props.progress | 0}%`}}/>
        </div>
      )}
    </div>
  );
}

export default CardImage;
