import { useContext, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";

import { formatHHMMSS } from "../../../Helpers/utils";
import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions/Index";
import CircleIcon from "../../../assets/Icons/Circle";

import "./Index.scss";

function VideoControls() {
  const { video, media } = useSelector((store) => ({
    video: store.video,
    media: store.media,
  }));

  const nameDiv = useRef(null);
  const timeDiv = useRef(null);

  const { seekTo, overlay } = useContext(VideoPlayerContext);
  const [visible, setVisible] = useState(true);

  useEffect(() => {
    if (!overlay) return;

    overlay.style.background = visible
      ? "linear-gradient(to top, #000, transparent 30%)"
      : "unset";
  }, [overlay, visible]);

  return (
    <div className={`videoControls ${visible}`}>
      <div className="name" ref={nameDiv}>
        <p>{media[video.mediaID]?.info.data.name}</p>
        {media[video.mediaID]?.info.data.episode && (
          <div className="season-ep">
            <p>S{media[video.mediaID]?.info.data.season}</p>
            <CircleIcon />
            <p>E{media[video.mediaID]?.info.data.episode}</p>
          </div>
        )}
      </div>
      <div className="time" ref={timeDiv}>
        <p>{formatHHMMSS(video.currentTime)}</p>
        <CircleIcon />
        <p>{formatHHMMSS(video.duration)}</p>
      </div>
      <SeekBar
        seekTo={seekTo}
        nameRef={nameDiv.current}
        timeRef={timeDiv.current}
      />
      <Actions setVisible={setVisible} seekTo={seekTo} />
    </div>
  );
}

export default VideoControls;
