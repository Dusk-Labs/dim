import { useCallback, useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import SeekingTo from "./SeekingTo";

import "./SeekBar.scss";
import { updateVideo } from "../../../actions/video";

function VideoSeekBar(props) {
  const dispatch = useDispatch();

  const { auth, video, player } = useSelector(store => ({
    auth: store.auth,
    video: store.video,
    player: store.video.player
  }));

  const seekBar = useRef(null);

  const seekBarCurrent = useRef(null);
  const bufferBar = useRef(null);

  const { seekTo } = props;
  const { token } = auth;

  // save progress every 15 seconds
  useEffect(() => {
    if (video.currentTime % 15 !== 0 || video.currentTime === 0) return;

    (async () => {
      const config = {
        method: "POST",
        headers: {
          "authorization": token
        }
      };

      console.log("[VIDEO] saving progress at", video.currentTime);

      await fetch(`/api/v1/media/${video.episode?.id || video.mediaID}/progress?offset=${video.currentTime}`, config);
    })();
  }, [video.currentTime, token, video.episode?.id, video.mediaID]);

  // current time
  useEffect(() => {
    const position = (video.currentTime / video.duration) * 100;
    seekBarCurrent.current.style.width = `${position}%`;
  }, [video.currentTime, video.duration]);

  // buffer
  useEffect(() => {
    const position = ((video.currentTime + video.buffer) / video.duration) * 100;
    bufferBar.current.style.width = `${position}%`;
  }, [video.currentTime, video.duration, video.buffer]);

  const onSeek = useCallback(async (e) => {
    if(video.seeking) return;

    dispatch(updateVideo({
      seeking: true
    }));

    const rect = e.target.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    const videoDuration = player.getMediaElement !== undefined ? player.getMediaElement().duration : player.duration();
    const newTime = Math.floor(percent * videoDuration);

    seekTo(newTime);
  }, [dispatch, player, seekTo, video.seeking]);

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
