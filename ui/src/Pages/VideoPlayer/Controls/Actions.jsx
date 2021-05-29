import { useCallback, useContext, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
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

import { toggleShowSubSwitcher, updateVideo } from "../../../actions/video";

import "./Actions.scss";

function VideoActions(props) {
  const dispatch = useDispatch();

  const { video, player } = useSelector(store => ({
    video: store.video,
    player: store.video.player
  }));

  const { videoPlayer } = useContext(VideoPlayerContext);
  const { seekTo, setVisible } = props;

  const [currentVolume, setCurrentVolume] = useState(100);

  const onVolumeChange = useCallback((e) => {
    const newVolume = e.target.value / 100;

    setCurrentVolume(newVolume * 100);
    player.setVolume(newVolume);
  }, [player]);

  const play = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    player.play();
  }, [dispatch, player]);

  const pause = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    player.pause();
  }, [dispatch, player]);

  const seekForward = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    if (video.currentTime + 15 >= video.duration) {
      seekTo(video.duration);
    } else {
      seekTo(video.currentTime + 15);
    }
  }, [dispatch, seekTo, video.currentTime, video.duration]);

  const seekBackward = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    if (video.currentTime - 15 <= 0) {
      seekTo(0);
    } else {
      seekTo(video.currentTime - 15);
    }
  }, [dispatch, seekTo, video.currentTime]);

  const toggleFullscreen = useCallback(async () => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    try {
      if (!video.fullscreen) {
        await videoPlayer.current.requestFullscreen();
      } else {
        await document.exitFullscreen();
      }
    } catch (e) {}
  }, [dispatch, video.fullscreen, videoPlayer]);

  const handleFullscreenChange = useCallback(() => {
    dispatch(updateVideo({
      fullscreen: document.fullscreenElement,
      idleCount: 0
    }));
  }, [dispatch]);

  const toggleMute = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    if (currentVolume === 0) {
      player.setMute(false);
      player.setVolume(1);
      setCurrentVolume(100);
    }

    if (currentVolume > 0) {
      const currentMuteState = player.isMuted();

      player.setMute(!currentMuteState);

      dispatch(updateVideo({
        muted: !currentMuteState
      }));
    }
  }, [currentVolume, dispatch, player]);

  const toggleSubtitles = useCallback(() => {
    dispatch(toggleShowSubSwitcher());
  }, [dispatch]);

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
    if (video.idleCount >= 2) {
      setVisible(false);
      document.getElementsByTagName("body")[0].style.cursor = "none";
    } else {
      setVisible(true);
      document.getElementsByTagName("body")[0].style.cursor = "default";
    }
  }, [video.idleCount, setVisible]);

  const showPlayer = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    setVisible(true);
    document.getElementsByTagName("body")[0].style.cursor = "default";
  }, [dispatch, setVisible]);

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
          {(!video.muted && currentVolume > 0) && <VolumeUpIcon/>}
          {(video.muted || currentVolume === 0) && <VolumeMuteIcon/>}
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
        {video.paused && (
          <button onClick={play} className="playpause">
            <PlayIcon/>
          </button>
        )}
        {!video.paused && (
          <button onClick={pause} className="playpause">
            <PauseIcon/>
          </button>
        )}
        <button onClick={seekForward} className="forward">
          <ForwardIcon/>
        </button>
      </section>
      <section className="right">
        <button onClick={toggleSubtitles} className={`cc trackActive-${video.textTrackEnabled} menuActive-${video.showSubSwitcher}`}>
          <CCIcon/>
        </button>
        <button onClick={toggleFullscreen} className="fullscreen">
          {video.fullscreen && <CompressIcon/>}
          {!video.fullscreen && <ExpandIcon/>}
        </button>
      </section>
    </div>
  );
}

export default VideoActions;
