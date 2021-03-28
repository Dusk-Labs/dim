import { useCallback, useContext, useEffect, useRef } from "react";
import { VideoPlayerContext } from "../Context";

import "./SeekBar.scss";

function VideoControls() {
  const { fileID, player, duration, currentTime, setCurrentTime, buffer, setBuffer } = useContext(VideoPlayerContext);

  const seekBarCurrent = useRef(null);
  const bufferBar = useRef(null);

  // current time
  useEffect(() => {
    const position = (currentTime / duration) * 100;
    seekBarCurrent.current.style.width = `${position}%`;
  }, [currentTime, duration])

  // buffer
  useEffect(() => {
    const position = ((currentTime + buffer) / duration) * 100;
    bufferBar.current.style.width = `${position}%`;
  }, [buffer, currentTime, duration])

  const onSeek = useCallback(async (e) => {
    if (seeking) return;
    setSeeking(true);

    const rect = e.target.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    const videoDuration = player.duration();
    const newTime = Math.floor(percent * videoDuration);
    const newSegment = Math.floor(newTime / 5);

    setCurrentTime(newTime);
    setBuffer(0);

    player.attachSource(`//${window.host}:8000/api/v1/stream/${fileID}/manifest.mpd?start_num=${newSegment}`);

    // setOldOffset(offset);
    // setCurrentTime(0);
    // setOffset(newTime);
    setSeeking(false);
  }, [fileID, player, seeking, setBuffer, setCurrentTime, setSeeking]);

  return (
    <div className="seekBar" onClick={onSeek}>
      <div ref={bufferBar} className="buffer"/>
      <div ref={seekBarCurrent} className="current"/>
    </div>
  );
}

export default VideoControls;
