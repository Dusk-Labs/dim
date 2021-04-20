import { useCallback, useContext, useEffect, useRef } from "react";
import { useSelector } from "react-redux";

import { VideoPlayerContext } from "../Context";
import SeekingTo from "./SeekingTo";

import "./SeekBar.scss";

function VideoSeekBar(props) {
  const auth = useSelector(store => store.auth);

  const seekBar = useRef(null);

  const { episode, mediaID, seeking, setSeeking, player, duration, currentTime, buffer } = useContext(VideoPlayerContext);

  const seekBarCurrent = useRef(null);
  const bufferBar = useRef(null);

  const { seekTo } = props;
  const { token } = auth;

  // save progress every 15 seconds
  useEffect(() => {
    if (currentTime % 15 !== 0 || currentTime === 0) return;

    (async () => {
      const config = {
        method: "POST",
        headers: {
            "authorization": token,
        }
      }

      console.log("saving progress");

      await fetch(`//${window.host}:8000/api/v1/media/${episode?.id || mediaID}/progress?offset=${currentTime}`, config);
    })();
  }, [currentTime, episode?.id, mediaID, token]);

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

    seekTo(newTime);
  }, [player, seekTo, seeking, setSeeking]);

  return (
    <div className="seekBarContainer">
      <div className="seekBar" onClick={onSeek} ref={seekBar}>
        <div ref={bufferBar} className="buffer"/>
        <div ref={seekBarCurrent} className="current"/>
      </div>
      <SeekingTo nameRef={props.nameRef} timeRef={props.timeRef} seekBar={seekBar}/>
    </div>
  );
}

export default VideoSeekBar;
