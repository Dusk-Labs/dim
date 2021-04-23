import { useContext, useEffect, useRef, useState } from "react";

import { formatHHMMSS } from "../../../Helpers/utils";
import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions";
import CircleIcon from "../../../assets/Icons/Circle";

import "./Index.scss";

/*
  logic for media name and other metadata is in place,
  awaiting info to be returned by API - hidden until then.
*/

function VideoControls() {
  const nameDiv = useRef(null);
  const timeDiv = useRef(null);

  const { mediaInfo, episode, seekTo, overlay, currentTime, duration } = useContext(VideoPlayerContext);
  const [ visible, setVisible ] = useState(true);

  useEffect(() => {
    if (!overlay) return;

    overlay.style.background = visible ? "linear-gradient(to top, #000, transparent 30%)" : "unset";
  }, [overlay, visible]);

  return (
    <div className={`videoControls ${visible}`}>
      <div className="name" ref={nameDiv}>
        <p>{mediaInfo.name}</p>
        {episode && (
          <div className="season-ep">
            <p>S{episode.season}</p>
            <CircleIcon/>
            <p>E{episode.episode}</p>
          </div>
        )}
      </div>
      <div className="time" ref={timeDiv}>
        <p>{formatHHMMSS(currentTime)}</p>
        <CircleIcon/>
        <p>{formatHHMMSS(duration)}</p>
      </div>
      <SeekBar seekTo={seekTo} nameRef={nameDiv.current} timeRef={timeDiv.current}/>
      <Actions setVisible={setVisible} seekTo={seekTo}/>
    </div>
  );
}

export default VideoControls;
