import { useCallback, useContext, useEffect, useState } from "react";

import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions";

import "./Index.scss";

function VideoControls() {
  const { setSeeking, player, currentTime, duration, setCurrentTime, setBuffer, fileID, mediaInfo } = useContext(VideoPlayerContext);
  const [ visible, setVisible ] = useState(true);
  const [ hideTimeoutID, setHideTimeoutID ] = useState();

  const seekTo = useCallback(async newTime => {
    const newSegment = Math.floor(newTime / 5);

    setCurrentTime(newTime);
    setBuffer(0);

    player.attachSource(`//${window.host}:8000/api/v1/stream/${fileID}/manifest.mpd?start_num=${newSegment}`);

    // setOldOffset(offset);
    // setCurrentTime(0);
    // setOffset(newTime);
    setSeeking(false);
  }, [fileID, player, setBuffer, setCurrentTime, setSeeking]);

  // converts to HH:MM:SS format
  const format = (secs) => (
    new Date(secs * 1000).toISOString().substr(11, 8)
  );

  const handleMouseMove = useCallback(() => {
    setVisible(true);
    clearTimeout(hideTimeoutID);

    document.getElementsByTagName("body")[0].style.cursor = "default";

    const ID = setTimeout(() => {
      document.getElementsByTagName("body")[0].style.cursor = "none";
      setVisible(false);
      setHideTimeoutID();
    }, 2000);

    setHideTimeoutID(ID);
  }, [hideTimeoutID]);

  useEffect(() => {
    document.addEventListener("mousemove", handleMouseMove);

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      clearTimeout(hideTimeoutID);
    }
  }, [handleMouseMove, hideTimeoutID]);

  return (
    <div className="videoControls">
      {visible && (
        <>
          <p className="name">{mediaInfo.name}</p>
          <p className="time">{format(currentTime)} - {format(duration)}</p>
          <SeekBar seekTo={seekTo}/>
          <Actions seekTo={seekTo}/>
        </>
      )}
    </div>
  );
}

export default VideoControls;
