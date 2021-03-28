import { useCallback, useContext } from "react";
import { VideoPlayerContext } from "../Context";

import PlayIcon from "../../../assets/Icons/Play";
import PauseIcon from "../../../assets/Icons/Pause";
import BackwardIcon from "../../../assets/Icons/Backward";
import ForwardIcon from "../../../assets/Icons/Forward";
import ExpandIcon from "../../../assets/Icons/Expand";
import CompressIcon from "../../../assets/Icons/Compress";
import VolumeUpIcon from "../../../assets/Icons/VolumeUp";
import VolumeMuteIcon from "../../../assets/Icons/VolumeMute";

import "./Index.scss";

function VideoActions(props) {
  const { muted, setMuted, videoPlayer, fullscreen, setFullscreen, currentTime, player, paused } = useContext(VideoPlayerContext);

  const { seekTo } = props;

  const play = useCallback(() => {
    player.play();
  }, [player]);

  const pause = useCallback(() => {
    player.pause();
  }, [player]);

  const seekForward = useCallback(() => {
    seekTo(currentTime + 15);
  }, [currentTime, seekTo]);

  const seekBackward = useCallback(() => {
    seekTo(currentTime - 15);
  }, [currentTime, seekTo]);

  const toggleFullscreen = useCallback(() => {
    setFullscreen(state => !state);

    if (!fullscreen) {
      videoPlayer.current.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  }, [fullscreen, setFullscreen, videoPlayer]);

  const toggleMute = useCallback(() => {
    const currentMuteState = player.isMuted();

    player.setMute(!currentMuteState);
    setMuted(!currentMuteState)
  }, [player, setMuted]);

  return (
    <div className="actions">
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
