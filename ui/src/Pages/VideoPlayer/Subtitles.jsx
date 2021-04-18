import { useCallback, useEffect, useState, useContext } from "react";
import { calcNewSize } from "../../Helpers/utils";
import { VideoPlayerContext } from "./Context";

import "./Subtitles.scss";

function VideoSubtitles() {
  const { currentTextTrack, textTrackEnabled, video, canPlay } = useContext(VideoPlayerContext);

  const [currentCue, setCurrentCue] = useState("");
  const [show, setShow] = useState(false);

   // relative to window width
   const updateBlackBarHeight = useCallback(() => {
    const videoHeight = calcNewSize(
      video.current.videoWidth,
      video.current.videoHeight,
      window.innerWidth
    );

    const blackBarHeight = (window.innerHeight - videoHeight) / 2;

    if (blackBarHeight > 100) {
      document.documentElement.style.setProperty("--blackBarHeight", `${blackBarHeight}px`);
    }
  }, [video]);

  const handleCueChange = useCallback((e) => {
    if (e.srcElement.activeCues.length > 0) {
      setCurrentCue(e.srcElement.activeCues[0].text);
      setShow(true);
    } else {
      setShow(false);
    }
  }, []);

  useEffect(() => {
    window.addEventListener("resize", updateBlackBarHeight);
    return () => window.removeEventListener("resize", updateBlackBarHeight);
  }, [updateBlackBarHeight]);

  useEffect(() => {
    if (!canPlay) return;
    updateBlackBarHeight();
  }, [canPlay, updateBlackBarHeight]);

  useEffect(() => {
    if (!video.current || !canPlay) return;

    const track = video.current.textTracks[currentTextTrack];

    if (track) {
      track.addEventListener("cuechange", handleCueChange);
    }
  }, [canPlay, currentTextTrack, handleCueChange, video]);

  return (
    <div className={`videoSubtitles show-${textTrackEnabled && show}`}>
      <p>{currentCue.replace(/<[^>]*>?/gm, "")}</p>
    </div>
  );
}


export default VideoSubtitles;
