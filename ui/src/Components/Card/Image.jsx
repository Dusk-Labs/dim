import { useRef, useState, useCallback, useEffect } from "react";
import DimLogo from "../../assets/DimLogo";
import ImageLoad from "../ImageLoad";

const CardImage = (props) => {
  const imageWrapperDiv = useRef(null);
  const [observed, setObserved] = useState(false);

  const handleIntersection = useCallback((entries, observer) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        setObserved(true);
        observer.disconnect();
      }
    });
  }, []);

  useEffect(() => {
    if (!imageWrapperDiv.current) return;

    const options = {
      root: null,
      rootMargin: "0px",
      threshold: 0,
    };

    const observer = new IntersectionObserver(handleIntersection, options);
    observer.observe(imageWrapperDiv.current);
    return () => observer.disconnect();
  }, [handleIntersection]);

  return (
    <div className="cardImageWrapper" ref={imageWrapperDiv}>
      {observed && (
        <ImageLoad src={props.src} triggerAnimation="onHideImage">
          {({ imageSrc, loaded, error, setErr }) => (
            <>
              {loaded && !error && (
                <img src={imageSrc} alt="cover" onError={() => setErr(true)} />
              )}
              {error && (
                <div className="placeholder">
                  <DimLogo />
                </div>
              )}
              {props.progress !== undefined && (
                <div className="progress">
                  <div
                    className="value"
                    style={{ width: `${props.progress | 0}%` }}
                  />
                </div>
              )}
            </>
          )}
        </ImageLoad>
      )}
    </div>
  );
};

export default CardImage;
