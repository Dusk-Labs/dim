import { useState, useEffect } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useCallback } from "react";

function LazyImage(props) {
  const [currentSrc, setCurrentSrc] = useState();
  const [show, setShow] = useState();

  const [loaded, setLoaded] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    if (!currentSrc) {
      setCurrentSrc(props.src);
    }

    if ((props.src !== currentSrc) && currentSrc) {
      setLoading(true);
      setLoaded(false);
      setErr(false);
      setShow(false);
    }
  }, [props.src]);

  const swapSrc = useCallback((e) => {
    if ((props.src !== currentSrc) && (e.animationName === props.hideAnimationName)) {
      setCurrentSrc(props.src);
      setShow(true);
    }
  }, [props.src]);

  // OK
  return (
    <div
      className={`image-wrapper show-${show && loaded}`}
      ref={props.imageWrapperRef}
      onAnimationEnd={swapSrc}
    >
      {loading && (
        <div className="placeholder">
          <div className="spinner"/>
        </div>
      )}
      {(loaded && error && !props.onFail) && (
        <div className="placeholder"/>
      )}
      {(loaded && error && !props.onFail && props.type === "SMALL") && (
        <div className="error-icon">
          <FontAwesomeIcon icon="times-circle"/>
        </div>
      )}
      <img
        key={props.src}
        src={currentSrc}
        alt={props.alt}
        onLoad={() => {
          setLoaded(true)
          setLoading(false)
        }}
        onError={() => {
          setLoaded(true);
          setLoading(false)
          setErr(true);
        }}/>
    </div>
  )
}

export default LazyImage;
