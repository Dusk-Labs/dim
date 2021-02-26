import { useState, useEffect, useCallback } from "react";

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
  }, [currentSrc, props.src])

  const swapSrc = useCallback((e) => {
    if (e.animationName !== "onHideProfileImage") return;

    setErr(false);

    if (props.src !== currentSrc) {
      const img = new Image();

      img.onload = async () => {
        setLoaded(true);
        setShow(true);
        setCurrentSrc(props.src);
      };

      img.onerror = () => {
        setLoaded(true);
        setShow(true);
        setErr(true);
      };

      img.src = props.src;
    }
  }, [currentSrc, props.src]);

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
          alt="Profile"
        />
      )}
    </div>
  );
}

export default ProfileImage;
