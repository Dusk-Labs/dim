import { useContext, useEffect, useRef, useState } from "react";

import { formatHHMMSS } from "../../../Helpers/utils";
import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions";

import "./Index.scss";

function VideoControls() {
  const nameDiv = useRef(null);
  const timeDiv = useRef(null);

  const { episode, seekTo, overlay, currentTime, duration, mediaInfo } = useContext(VideoPlayerContext);
  const [ visible, setVisible ] = useState(true);

  useEffect(() => {
    if (!overlay) return;

    overlay.style.background = visible ? "linear-gradient(to top, #000, transparent 30%)" : "unset";
  }, [overlay, visible])

  return (
    <div className={`videoControls ${visible}`}>
      <p className="name" ref={nameDiv}>
        {mediaInfo.name}{episode && `- S${episode.season} E${episode.episode}`}
      </p>
      <p className="time" ref={timeDiv}>
        {formatHHMMSS(currentTime)} - {formatHHMMSS(duration)}
      </p>
      <SeekBar seekTo={seekTo} nameRef={nameDiv.current} timeRef={timeDiv.current}/>
      <Actions setVisible={setVisible} seekTo={seekTo}/>
    </div>
  );
}

export default VideoControls;
