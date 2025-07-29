import { useEffect, useState, useContext } from "react";
import { useSelector } from "react-redux";

import { VideoPlayerContext } from "./Context";

import JASSUB from "jassub";

import "./Subtitles.scss";

function VideoSubtitles() {
  const { video, subtitle } = useSelector((store) => ({
    video: store.video,
    subtitle: store.video.tracks.subtitle,
  }));

  const currentSub = subtitle.list[subtitle.current];

  const isAssEnabled = localStorage.getItem("enable_ssa") === "true";
  const isAss = !!(isAssEnabled && currentSub?.chunk_path?.endsWith("ass"));
  const [jassub, setJASSUB] = useState();
  const { videoRef } = useContext(VideoPlayerContext);

  useEffect(() => {
    if (
      jassub ||
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

    JASSUB._test();

    const options = {
      video: videoRef.current,
      subUrl: chunk_path,
      dropAllBlur: !JASSUB._supportsSIMD,
      workerUrl: new URL(
        "jassub/dist/jassub-worker.js",
        import.meta.url
      ).toString(),
      wasmUrl: new URL(
        "jassub/dist/jassub-worker.wasm",
        import.meta.url
      ).toString(),
      modernWasmUrl: new URL(
        "jassub/dist/jassub-worker-modern.wasm",
        import.meta.url
      ).toString(),
      availableFonts: { "liberation sans": "/static/default.woff2" },
      fonts: ["/static/default.woff2"],
    };

    setJASSUB(new JASSUB(options));

    return () => {
      console.log("[subtitle] disposing of jassub ctx");
      if (jassub) jassub.destroy();
    };
  }, [video, videoRef, subtitle, isAss, setJASSUB, jassub]);

  useEffect(() => {
    if (
      !jassub ||
      !video.textTrackEnabled ||
      video.prevSubs === subtitle.current ||
      !isAss
    )
      return;

    const chunk_path = `//${window.location.host}/api/v1/stream/${
      subtitle.list[subtitle.current].chunk_path
    }`;
    jassub.setTrackByUrl(chunk_path);
  }, [jassub, video.textTrackEnabled, video.prevSubs, subtitle, isAss]);

  useEffect(() => {
    if (jassub && !isAss) {
      console.log("[subtitle] disposing of jassub ctx");
      jassub.destroy();
      setJASSUB(null);
    }
  }, [jassub, setJASSUB, isAss]);

  return null;
}

export default VideoSubtitles;
