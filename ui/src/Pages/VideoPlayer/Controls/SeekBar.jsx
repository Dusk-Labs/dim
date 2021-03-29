import { useCallback, useContext, useEffect, useRef } from "react";
import { VideoPlayerContext } from "../Context";

import "./SeekBar.scss";

function VideoSeekBar(props) {
  const { seeking, setSeeking, player, duration, currentTime, buffer } = useContext(VideoPlayerContext);

  const seekBarCurrent = useRef(null);
  const bufferBar = useRef(null);

  const { seekTo } = props;

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

  useEffect(() => {
    const savedCurrentTime = sessionStorage.getItem("currentTime");

    if (savedCurrentTime) {
      seekTo(savedCurrentTime);
      sessionStorage.clear();
    }
  }, [seekTo]);

  const onSeek = useCallback(async (e) => {
    if (seeking) return;

    setSeeking(true);

    const rect = e.target.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    const videoDuration = player.duration();
    const newTime = Math.floor(percent * videoDuration);

    seekTo(newTime);
  }, [player, seekTo, seeking, setSeeking]);

  return (
    <div className="seekBar" onClick={onSeek}>
      <div ref={bufferBar} className="buffer"/>
      <div ref={seekBarCurrent} className="current"/>
    </div>
  );
}

export default VideoSeekBar;
