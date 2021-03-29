import { useCallback, useContext, useEffect, useState } from "react";
import { VideoPlayerContext } from "../Context";

import PlayIcon from "../../../assets/Icons/Play";
import PauseIcon from "../../../assets/Icons/Pause";
import BackwardIcon from "../../../assets/Icons/Backward";
import ForwardIcon from "../../../assets/Icons/Forward";
import ExpandIcon from "../../../assets/Icons/Expand";
import CompressIcon from "../../../assets/Icons/Compress";
import VolumeUpIcon from "../../../assets/Icons/VolumeUp";
import VolumeMuteIcon from "../../../assets/Icons/VolumeMute";

import "./Actions.scss";

function VideoActions(props) {
  const { duration, muted, setMuted, videoPlayer, fullscreen, setFullscreen, currentTime, player, paused } = useContext(VideoPlayerContext);
  const { seekTo, setVisible } = props;

  const [ idleCount, setIdleCount ] = useState(0);

  const play = useCallback(() => {
    setIdleCount(0);
    player.play();
  }, [player]);

  const pause = useCallback(() => {
    setIdleCount(0);
    player.pause();
  }, [player]);

  const seekForward = useCallback(() => {
    setIdleCount(0);

    if (currentTime + 15 >= duration) {
      seekTo(duration);
    } else {
      seekTo(currentTime + 15);
    }
  }, [currentTime, duration, seekTo]);

  const seekBackward = useCallback(() => {
    setIdleCount(0);

    if (currentTime - 15 <= 0) {
      seekTo(0);
    } else {
      seekTo(currentTime - 15);
    }
  }, [currentTime, seekTo]);

  const toggleFullscreen = useCallback(async () => {
    setIdleCount(0);

    try {
      if (!fullscreen) {
        await videoPlayer.current.requestFullscreen()
      } else {
        await document.exitFullscreen();
      }
    } catch (e) {}
  }, [fullscreen, videoPlayer]);

  const handleFullscreenChange = useCallback(() => {
    setIdleCount(0);
    setFullscreen(document.fullscreenElement)
  }, [setFullscreen]);

  const toggleMute = useCallback(() => {
    setIdleCount(0);
    const currentMuteState = player.isMuted();

    player.setMute(!currentMuteState);
    setMuted(!currentMuteState)
  }, [player, setMuted]);

  const handleKeyDown = useCallback(e => {
    if (e.key === "f") {
      toggleFullscreen();
    }

    if (e.key === "ArrowLeft") {
      seekBackward();
    }

    if (e.key === "ArrowRight") {
      seekForward();
    }

    if (e.key === " ") {
      if (player.isPaused()) {
        play();
      } else {
        pause();
      }
    }

    if (e.key === "m") {
      toggleMute();
    }
  }, [pause, play, player, seekBackward, seekForward, toggleFullscreen, toggleMute]);

  useEffect(() => {
    if (idleCount >= 2) {
      setVisible(false);
      document.getElementsByTagName("body")[0].style.cursor = "none";
    } else {
      setVisible(true);
      document.getElementsByTagName("body")[0].style.cursor = "default";
    }

    setIdleCount(state => state += 1);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentTime]);

  const showPlayer = useCallback(() => {
    setIdleCount(0);
    setVisible(true);
    document.getElementsByTagName("body")[0].style.cursor = "default";
  }, [setVisible]);

  useEffect(() => {
    document.addEventListener("mousemove", showPlayer);
    document.addEventListener("fullscreenchange", handleFullscreenChange);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("mousemove", showPlayer);
      document.removeEventListener("fullscreenchange", handleFullscreenChange);
      document.removeEventListener("keydown", handleKeyDown);
    }
  }, [handleFullscreenChange, handleKeyDown, showPlayer]);

  return (
    <div className="videoActions">
      <button onClick={toggleMute} className="volume">
        {!muted && <VolumeUpIcon/>}
        {muted && <VolumeMuteIcon/>}
      </button>
      <button onClick={seekBackward} className="backward">
        <BackwardIcon/>
      </button>
      {paused && (
        <button onClick={play} className="playpause">
          <PlayIcon/>
        </button>
      )}
      {!paused && (
        <button onClick={pause} className="playpause">
          <PauseIcon/>
        </button>
      )}
      <button onClick={seekForward} className="forward">
        <ForwardIcon/>
      </button>
      <button onClick={toggleFullscreen} className="fullscreen">
        {fullscreen && <CompressIcon/>}
        {!fullscreen && <ExpandIcon/>}
      </button>
    </div>
  );
}

export default VideoActions;
