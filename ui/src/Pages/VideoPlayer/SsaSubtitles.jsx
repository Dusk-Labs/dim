import { useEffect, useState, useContext } from "react";
import { useSelector } from "react-redux";

import { VideoPlayerContext } from "./Context";

import SubtitleOctopus from "@jellyfin/libass-wasm/dist/js/subtitles-octopus";

import "./Subtitles.scss";

function VideoSubtitles() {
  const { video, subtitle } = useSelector((store) => ({
    video: store.video,
    subtitle: store.video.tracks.subtitle,
  }));

  const currentSub = subtitle.list[subtitle.current];

  const isAssEnabled = localStorage.getItem("enable_ssa") === "true";
  const isAss = !!(isAssEnabled && currentSub?.chunk_path?.endsWith("ass"));
  const [octopus, setOctopus] = useState();
  const { videoRef } = useContext(VideoPlayerContext);

  useEffect(() => {
    if (
      octopus ||
      !video.textTrackEnabled ||
      video.prevSubs === subtitle.current ||
      !isAss ||
      !videoRef
    )
      return;

    console.log("[INFO] Loading ASS subtitle");

    const chunk_path = `//${window.location.host}/api/v1/stream/${
      subtitle.list[subtitle.current].chunk_path
    }`;
    const options = {
      video: videoRef.current,
      subUrl: chunk_path,
      workerUrl: "/static/subtitles-octopus-worker.js",
    };

    setOctopus(new SubtitleOctopus(options));

    return () => {
      console.log("[subtitle] disposing of octopus ctx");
      if (octopus) octopus.dispose();
    };
  }, [video, videoRef, subtitle, isAss, setOctopus, octopus]);

  useEffect(() => {
    if (
      !octopus ||
      !video.textTrackEnabled ||
      video.prevSubs === subtitle.current ||
      !isAss
    )
      return;

    const chunk_path = `//${window.location.host}/api/v1/stream/${
      subtitle.list[subtitle.current].chunk_path
    }`;
    octopus.setTrackByUrl(chunk_path);
  }, [octopus, video.textTrackEnabled, video.prevSubs, subtitle, isAss]);

  useEffect(() => {
    if (octopus && !isAss) {
      console.log("[subtitle] disposing of octopus ctx");
      octopus.dispose();
      setOctopus(null);
    }
  }, [octopus, setOctopus, isAss]);

  return null;
}

export default VideoSubtitles;
