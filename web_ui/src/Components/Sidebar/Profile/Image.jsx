import React, { useState, useEffect, useCallback } from "react";

import "./Image.scss";

function ProfileImage(props) {
  const [currentSrc, setCurrentSrc] = useState();
  const [show, setShow] = useState(false);

  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    if (props.src !== currentSrc) {
      setShow(false);
      setLoaded(false);
      setErr(false);
    }
  }, [props.src])

  const swapSrc = useCallback((e) => {
    if (e.animationName !== "onHideProfileImage") return;

    setErr(false);

    if (props.src !== currentSrc) {
      const img = new Image();

      img.onload = async () => {
        console.log("image loaded");
        setLoaded(true);
        setShow(true);
        setCurrentSrc(props.src);
      };

      img.onerror = () => {
        console.log("image error");
        setLoaded(true);
        setShow(true);
        setErr(true);
      };

      img.src = props.src;
    }
  }, [props.src]);

  if (error) {
    return (
      <div className="placeholder-icon"/>
    )
  }

  return (
    <div
      className={`profileImage show-${show && loaded}`}
      onAnimationEnd={swapSrc}
    >
      {(!error && loaded) && (
        <img
          src={currentSrc}
          key={currentSrc}
          alt="Profile Image"
        />
      )}
    </div>
  );
}

export default ProfileImage;