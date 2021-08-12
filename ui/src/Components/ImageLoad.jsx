import { useState, useEffect, useCallback } from "react";

function ImageLoad(props) {
  const [show, setShow] = useState(false);
  const [signal, setSignal] = useState();
  const [timeoutID, setTimeoutID] = useState();

  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  const [tryAgain, setTryAgain] = useState(false);
  const [tryAgainCount, setTryAgainCount] = useState(2);

  const [imageSrc, setImageSrc] = useState();
  const [currentSrc, setCurrentSrc] = useState();

  /*
    the difference in time between the imageObject being set and
    rendering the component is enough for smoother onShow animations
  */
  useEffect(() => {
    if (imageSrc) {
      setShow(true);
    }
  }, [imageSrc]);

  const fetchImage = useCallback(async () => {
    setLoaded(false);
    setImageSrc();
    setTryAgain(false);
    setTimeoutID();

    const src = new RegExp("/^(?:/|[a-z]+://)/").test(props.src) ? props.src : `/${props.src}`;

    try {
      const req = await fetch(src, {signal});
      const blob = await req.blob();

      setLoaded(true);
      setCurrentSrc(props.src);

      if (blob.type === "text/html; charset=utf-8" || req.status !== 200) {
        setShow(true);
        setErr(true);

        /*
          prevents trying to re-fetch every time
          the user navigates or reloads a page.
        */
        if (tryAgainCount > 0) {
          const triedAlready = sessionStorage.getItem(props.src);

          if (!triedAlready) {
            setTryAgain(true);
          }
        } else {
          sessionStorage.setItem(props.src, "skip");
        }

        return;
      }

      const imageObjectURL = URL.createObjectURL(blob);

      setImageSrc(imageObjectURL);
      setErr(false);
    } catch (e) {
      setErr(true);
      setShow(true);

      console.log("[img] unexpected error:", e);
    }
  }, [props.src, signal, tryAgainCount]);

  useEffect(() => {
    if (tryAgain && !timeoutID) {
      setTryAgainCount(state => state - 1);

      const id = setTimeout(() => {
        fetchImage();
      }, 3000);

      setTimeoutID(id);
    }

    return () => {
      if (timeoutID) {
        clearTimeout(timeoutID);
      }
    };
  }, [fetchImage, timeoutID, tryAgain]);

  const handleAnimationEnd = useCallback((e) => {
    if (e.animationName !== props.triggerAnimation) return;

    fetchImage();
    setTryAgainCount(2);
  }, [fetchImage, props.triggerAnimation]);

  useEffect(() => {
    // setShow(true);
    if (props.src === currentSrc) return;
    setShow(false);
  }, [currentSrc, props.src]);

  useEffect(() => {
    const controller = new AbortController();
    const acSignal = controller.signal;

    setSignal(acSignal);

    return () => {
      controller.abort();
    };
  }, []);

  if (!signal) return null;

  return (
    <div
      className={`imageLoad show-${show}`}
      onAnimationEnd={handleAnimationEnd}
    >
      {props.children(imageSrc, loaded, error)}
    </div>
  );
}

export default ImageLoad;
