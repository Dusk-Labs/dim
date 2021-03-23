import { useCallback, useContext } from "react";
import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";

import PlayIcon from "../../../assets/Icons/Play";
import PauseIcon from "../../../assets/Icons/Pause";

import "./Index.scss";

function VideoControls() {
  const { player, currentTime, duration, paused } = useContext(VideoPlayerContext);

  const play = useCallback(() => {
    player.play();
  }, [player]);

  const pause = useCallback(() => {
    player.pause();
  }, [player]);

  // converts to HH:MM:SS format
  const format = (secs) => (
    new Date(secs * 1000).toISOString().substr(11, 8)
  );

  return (
    <div className="videoControls">
      <p className="name">Media name</p>
      <p className="time">{format(currentTime)} - {format(duration)}</p>
      <SeekBar/>
      <div className="actions">
        {paused && (
          <button onClick={play}>
            <PlayIcon/>
          </button>
        )}
        {!paused && (
          <button onClick={pause}>
            <PauseIcon/>
          </button>
        )}
      </div>
    </div>
  );
}

export default VideoControls;
