import { useCallback, useEffect, useState, useContext, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { VideoPlayerContext } from "./Context";
import { updateTrack, updateVideo } from "../../actions/video";

import SubtitleOctopus from "libass-wasm/dist/js/subtitles-octopus";

import "./Subtitles.scss";

function VideoSubtitles() {
  const dispatch = useDispatch();

  const { video, player, subtitle } = useSelector(store => ({
    video: store.video,
    player: store.video.player,
    subtitle: store.video.tracks.subtitle
  }));

  const isAss = subtitle.list[subtitle.current]?.chunk_path?.endsWith("ass");
  const [octopus, setOctopus] = useState();
  const { videoRef } = useContext(VideoPlayerContext);

  useEffect(() => {
    if (octopus || !video.textTrackEnabled || video.prevSubs === subtitle.current || !isAss || !videoRef) return;

    const chunk_path = `//${window.location.host}/api/v1/stream/${subtitle.list[subtitle.current].chunk_path}`;
    const options = {
      "video": videoRef.current,
      "subUrl": chunk_path,
      "workerUrl": "/static/subtitles-octopus-worker.js"
    };

    setOctopus(new SubtitleOctopus(options));
  }, [video, dispatch, videoRef, subtitle, isAss, setOctopus]);

  return (
    <>
    </>
  );
}

export default VideoSubtitles;
