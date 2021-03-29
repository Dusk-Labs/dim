import { useCallback, useContext, useState } from "react";

import { VideoPlayerContext } from "../Context";
import SeekBar from "./SeekBar";
import Actions from "./Actions";

import "./Index.scss";

function VideoControls() {
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

  // converts to HH:MM:SS format
  const format = (secs) => (
    new Date(secs * 1000).toISOString().substr(11, 8)
  );

  return (
    <div className={`videoControls ${visible}`}>
      <p className="name">{mediaInfo.name}</p>
      <p className="time">{format(currentTime)} - {format(duration)}</p>
      <SeekBar seekTo={seekTo}/>
      <Actions setVisible={setVisible} seekTo={seekTo}/>
    </div>
  );
}

export default VideoControls;
