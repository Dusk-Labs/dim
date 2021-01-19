import React, { useState, useEffect } from "react";

function BannerImage(props) {
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
    <div className="bannerImageWrapper">
      {(loaded && !error) && (
        <img src={props.src} alt="cover"/>
      )}
      {error && (
        <div className="placeholder"/>
      )}
    </div>
  )
}

export default BannerImage;
