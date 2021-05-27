import { useCallback, useEffect, useRef } from "react";
import { useContext } from "react";

import { VideoPlayerContext } from "./Context";

function VideoMenus() {
  const { idleCount, setSubReady, setTextTrackEnabled, setShowSubSelection, currentSubtitleTrack, subtitleTracks, setCurrentSubtitleTrack } = useContext(VideoPlayerContext);

  const menuRef = useRef(null);

  const changeTrack = useCallback((i) => {
    console.log(currentSubtitleTrack, i);
    if (currentSubtitleTrack === i) return;
    setSubReady(false);
    setCurrentSubtitleTrack(i);
    setTextTrackEnabled(true);
    setShowSubSelection(false);
  }, [currentSubtitleTrack, setCurrentSubtitleTrack, setShowSubSelection, setSubReady, setTextTrackEnabled]);

  const turnOffSubs = useCallback(() => {
    if (currentSubtitleTrack === -1) return;
    console.log("[Subtitles] turning off subs");
    setTextTrackEnabled(false);
    setCurrentSubtitleTrack(-1);
  }, [currentSubtitleTrack, setCurrentSubtitleTrack, setTextTrackEnabled]);

  const handleClick = useCallback((e) => {
    if (!menuRef.current) return;

    if (!menuRef.current.contains(e.target)) {
      setShowSubSelection(false);
    }
  }, [setShowSubSelection]);

  useEffect(() => {
    if (idleCount >= 2) {
      setShowSubSelection(false);
    }
  }, [idleCount, setShowSubSelection]);

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    };
  }, [handleClick]);

  return (
    <div className="videoMenus">
      <div className="menu" ref={menuRef}>
        <h3>Select subtitle</h3>
        <div className="separator"/>
        <div className="tracks">
          <div className={`track ${currentSubtitleTrack === -1 ? "active" : ""}`} onClick={turnOffSubs}>
            <p>Off</p>
          </div>
          {subtitleTracks.map((track, i) => (
            <div key={i} className={`track ${currentSubtitleTrack === i ? "active" : ""}`} onClick={() => changeTrack(i)}>
              <p>{track.title || "No title"}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default VideoMenus;
