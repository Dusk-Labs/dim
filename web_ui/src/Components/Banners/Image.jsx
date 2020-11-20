import React, { useState, useEffect, useCallback } from "react";
import * as Vibrant from "node-vibrant";

function BannerImage(props) {
  const [currentSrc, setCurrentSrc] = useState();
  const [show, setShow] = useState(false);

  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    if (props.src !== currentSrc) {
      setShow(false);
      setLoaded(false);
    }
  }, [props.src])

  const swapSrc = useCallback((e) => {
    if (e.animationName !== props.hideAnimationName) return;

    setErr(false);

    if (props.src !== currentSrc) {
      const img = new Image();

      img.onload = async (e) => {
        setLoaded(true);
        setShow(true);
        setCurrentSrc(props.src);

        const color = await Vibrant.from(e.target).getPalette();

        props.setBG(color.Vibrant.getHex());
        props.setText(color.Vibrant.getTitleTextColor());
      };

      img.onerror = () => {
        setLoaded(true);
        setShow(true);
        setErr(true);
      };

      img.src = props.src;
    }
  }, [props.src]);

  return (
    <div
      className={`image-wrapper show-${show && loaded}`}
      onAnimationEnd={swapSrc}
    >
      {!error && (
        <img
          src={currentSrc}
          key={currentSrc}
          alt="banner"
        />
      )}
      {error && <div className="placeholder"/>}
    </div>
  )
}

export default BannerImage;
