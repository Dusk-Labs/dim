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
import CCIcon from "../../../assets/Icons/CC";

import "./Actions.scss";

function VideoActions(props) {
  const { textTrackEnabled, setTextTrackEnabled, duration, muted, setMuted, videoPlayer, fullscreen, setFullscreen, currentTime, player, paused } = useContext(VideoPlayerContext);
  const { seekTo, setVisible } = props;

  const [currentVolume, setCurrentVolume] = useState(100);
  const [idleCount, setIdleCount] = useState(0);

  const onVolumeChange = useCallback((e) => {
    const newVolume = e.target.value / 100;

    setCurrentVolume(newVolume * 100);
    player.setVolume(newVolume);
  }, [player]);

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
        await videoPlayer.current.requestFullscreen();
      } else {
        await document.exitFullscreen();
      }
    } catch (e) {}
  }, [fullscreen, videoPlayer]);

  const handleFullscreenChange = useCallback(() => {
    setIdleCount(0);
    setFullscreen(document.fullscreenElement);
  }, [setFullscreen]);

  const toggleMute = useCallback(() => {
    setIdleCount(0);

    if (currentVolume === 0) {
      player.setMute(false);
      player.setVolume(1);
      setCurrentVolume(100);
    }

    if (currentVolume > 0) {
      const currentMuteState = player.isMuted();

      player.setMute(!currentMuteState);
      setMuted(!currentMuteState);
    }
  }, [currentVolume, player, setMuted]);

  const toggleSubtitles = useCallback(() => {
    setTextTrackEnabled(state => !state);
  }, [setTextTrackEnabled]);

  useEffect(() => {
    player.enableText(textTrackEnabled);

    if (textTrackEnabled) {
      player.setTextTrack(0);
    }
  }, [player, textTrackEnabled]);

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

    if (e.key === "c") {
      toggleSubtitles();
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
  }, [pause, play, player, seekBackward, seekForward, toggleFullscreen, toggleMute, toggleSubtitles]);

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
    };
  }, [handleFullscreenChange, handleKeyDown, showPlayer]);

  return (
    <div className="videoActions">
      <section className="left">
        <button onClick={toggleMute} className="volume">
          {(!muted && currentVolume > 0) && <VolumeUpIcon/>}
          {(muted || currentVolume === 0) && <VolumeMuteIcon/>}
        </button>
        <input
          className="volumeSlider"
          type="range"
          min="0"
          max="100"
          value={currentVolume}
          onChange={onVolumeChange}
        />
      </section>
      <section className="middle">
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
      </section>
      <section className="right">
        <button onClick={toggleSubtitles} className={`cc active-${textTrackEnabled}`}>
          <CCIcon/>
        </button>
        <button onClick={toggleFullscreen} className="fullscreen">
          {fullscreen && <CompressIcon/>}
          {!fullscreen && <ExpandIcon/>}
        </button>
      </section>
    </div>
  );
}

export default VideoActions;
