import { useContext, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";

import { formatHHMMSS } from "../../../Helpers/utils";
import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions/Index";
import CircleIcon from "../../../assets/Icons/Circle";

import "./Index.scss";

function VideoControls() {
  const { video, mediaInfo } = useSelector(store => ({
    video: store.video,
    mediaInfo: store.card.media_info.info
  }));

  const nameDiv = useRef(null);
  const timeDiv = useRef(null);

  const { seekTo, overlay } = useContext(VideoPlayerContext);
  const [visible, setVisible] = useState(true);

  useEffect(() => {
    if (!overlay) return;

    overlay.style.background = visible ? "linear-gradient(to top, #000, transparent 30%)" : "unset";
  }, [overlay, visible]);

  return (
    <div className={`videoControls ${visible}`}>
      <div className="name" ref={nameDiv}>
        <p>{mediaInfo.name}</p>
        {video.episode && (
          <div className="season-ep">
            <p>S{video.episode.season}</p>
            <CircleIcon/>
            <p>E{video.episode.episode}</p>
          </div>
        )}
      </div>
      <div className="time" ref={timeDiv}>
        <p>{formatHHMMSS(video.currentTime)}</p>
        <CircleIcon/>
        <p>{formatHHMMSS(video.duration)}</p>
      </div>
      <SeekBar seekTo={seekTo} nameRef={nameDiv.current} timeRef={timeDiv.current}/>
      <Actions setVisible={setVisible} seekTo={seekTo}/>
    </div>
  );
}

export default VideoControls;
