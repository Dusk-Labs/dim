import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import VolumeUpIcon from "../../../../assets/Icons/VolumeUp";
import VolumeMuteIcon from "../../../../assets/Icons/VolumeMute";

import { updateVideo } from "../../../../actions/video";

function VideoActionVolume() {
  const dispatch = useDispatch();

  const { video, player } = useSelector(store => ({
    video: store.video,
    player: store.video.player
  }));

  const [currentVolume, setCurrentVolume] = useState(100);

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

  const onVolumeChange = useCallback((e) => {
    const newVolume = e.target.value / 100;

    setCurrentVolume(newVolume * 100);
    player.setVolume(newVolume);
  }, [player]);

  const handleKeyDown = useCallback(e => {
    if (e.key === "m") {
      toggleMute();
    }
  }, [toggleMute]);

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  let icon;

  if (!video.muted && currentVolume > 0) icon = (
    <button onClick={toggleMute} className="volume">
      <VolumeUpIcon/>
    </button>
  );

  else icon = (
    <button onClick={toggleMute} className="volume">
      <VolumeMuteIcon/>
    </button>
  );

  return (
    <>
      {icon}
      <input
        className="volumeSlider"
        type="range"
        min="0"
        max="100"
        value={currentVolume}
        onChange={onVolumeChange}
      />
    </>
  );
}

export default VideoActionVolume;
