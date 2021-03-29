import { useCallback, useContext, useRef, useState } from "react";

import { formatHHMMSS } from "../../../Helpers/utils";
import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions";

import "./Index.scss";

function VideoControls() {
  const nameDiv = useRef(null);
  const timeDiv = useRef(null);

  const { videoUUID, setSeeking, player, currentTime, duration, setCurrentTime, setBuffer, fileID, mediaInfo } = useContext(VideoPlayerContext);
  const [ visible, setVisible ] = useState(true);

  const seekTo = useCallback(async newTime => {
    const newSegment = Math.floor(newTime / 5);

    setCurrentTime(newTime);
    setBuffer(0);

    player.attachSource(`//${window.host}:8000/api/v1/stream/${fileID}/manifest.mpd?start_num=${newSegment}&gid=${videoUUID}`);

    // setOldOffset(offset);
    // setCurrentTime(0);
    // setOffset(newTime);
    setSeeking(false);
  }, [fileID, player, setBuffer, setCurrentTime, setSeeking, videoUUID]);

  return (
    <div className={`videoControls ${visible}`}>
      <p className="name" ref={nameDiv}>{mediaInfo.name}</p>
      <p className="time" ref={timeDiv}>{formatHHMMSS(currentTime)} - {formatHHMMSS(duration)}</p>
      <SeekBar seekTo={seekTo} nameRef={nameDiv.current} timeRef={timeDiv.current}/>
      <Actions setVisible={setVisible} seekTo={seekTo}/>
    </div>
  );
}

export default VideoControls;
