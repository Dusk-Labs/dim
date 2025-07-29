import { useState, useEffect, useCallback } from "react";

import "./Image.scss";

interface Props {
  src: string;
}

function ProfileImage(props: Props) {
  const [currentSrc, setCurrentSrc] = useState<string | null>(null);
  const [show, setShow] = useState(false);

  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    // If the server returned no profile pic we want to just show a default fill without making a null request.
    if (!props.src) {
      setShow(true);
      setLoaded(true);
      setErr(true);
      return;
    }

    if (props.src !== currentSrc) {
      setShow(false);
      setLoaded(false);
      setErr(false);
    }
  }, [currentSrc, props.src]);

  const swapSrc = useCallback(
    (e) => {
      if (e.animationName !== "onHideProfileImage") return;

      setErr(false);

      if (props.src && props.src !== currentSrc) {
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
    },
    [currentSrc, props.src]
  );

  return (
    <div
      className={`profileImage show-${show && loaded}`}
      onAnimationEnd={swapSrc}
    >
      {!error && loaded && currentSrc && (
        <img src={currentSrc} key={currentSrc} alt="Profile" />
      )}
      {error && loaded && <div className="placeholder" />}
    </div>
  );
}

export default ProfileImage;
