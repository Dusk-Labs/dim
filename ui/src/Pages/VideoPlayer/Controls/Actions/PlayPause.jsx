import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import PlayIcon from "../../../../assets/Icons/Play";
import PauseIcon from "../../../../assets/Icons/Pause";

import { updateVideo } from "../../../../actions/video";

function VideoActionPlayPause() {
  const dispatch = useDispatch();

  const { video, player } = useSelector(store => ({
    video: store.video,
    player: store.video.player
  }));

  const play = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    player.getMediaElement().play();
  }, [dispatch, player]);

  const pause = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    player.getMediaElement().pause();
  }, [dispatch, player]);

  const handleKeyDown = useCallback(e => {
    if (e.key !== " ") return;
    if (e.target !== document.body) return;

    player.isPaused() ? play() : pause();
  }, [pause, play, player]);

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <button
      onClick={video.paused ? play : pause}
      className="playpause"
    >
      {video.paused ? <PlayIcon/> : <PauseIcon/>}
    </button>
  );
}

export default VideoActionPlayPause;
