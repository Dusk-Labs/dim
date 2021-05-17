import { useCallback, useEffect, useState, useContext } from "react";
import { calcNewSize } from "../../Helpers/utils";
import { VideoPlayerContext } from "./Context";

import "./Subtitles.scss";

function VideoSubtitles() {
  const { subtitleTracks, currentSubtitleTrack, player, currentTextTrack, textTrackEnabled, video, canPlay } = useContext(VideoPlayerContext);

  const [currentCue, setCurrentCue] = useState("");
  const [show, setShow] = useState(false);
  const [subReady, setSubReady] = useState(false);

  // relative to window width
  const updateBlackBarHeight = useCallback(() => {
    const videoHeight = calcNewSize(
      video.current.videoWidth,
      video.current.videoHeight,
      window.innerWidth
    );

    const blackBarHeight = (window.innerHeight - videoHeight) / 2;

    if (blackBarHeight > 100) {
      document.documentElement.style.setProperty("--blackBarHeight", `${blackBarHeight}px`);
    }
  }, [video]);

  const handleCueChange = useCallback((e) => {
    if (e.srcElement.activeCues.length > 0) {
      setCurrentCue(e.srcElement.activeCues[0].text);
      setShow(true);
    } else {
      setShow(false);
    }
  }, []);

  useEffect(() => {
    if (subtitleTracks.length === 0 || !textTrackEnabled) return;

    let prevCount = 0;

    console.log("[Subtitles] preparing subtitle track", subtitleTracks[currentSubtitleTrack]);

    const intervalID = setInterval(async () => {
      const req = await fetch(`/api/v1/stream/${subtitleTracks[currentSubtitleTrack].id}/data/stream.vtt`);
      const text = await req.text();

      if (!text || (text && text.length === prevCount)) {
        console.log("[Subtitles] subtitles fully loaded");
        clearInterval(intervalID);
        setSubReady(true);
      } else {
        console.log("[Subtitles] partially loaded, re-fetching again in 3 seconds");
        prevCount = text.length;
      }
    }, 3000);
  }, [currentSubtitleTrack, subtitleTracks, textTrackEnabled]);

  useEffect(() => {
    if (!subReady || !player) return;

    console.log("[Subtitles] ready to show subtitles", currentSubtitleTrack);

    player.setTextTrack(currentSubtitleTrack);
  }, [currentSubtitleTrack, player, subReady, textTrackEnabled]);

  useEffect(() => {
    if (!player) return;
    console.log("[Subtitles] setting player text status to", textTrackEnabled);
    player.enableText(textTrackEnabled);
  }, [player, textTrackEnabled]);

  useEffect(() => {
    if (!player) return;

    setInterval(() => {
      console.log("[Subtitles] is enabled?", player.isTextEnabled());
    }, 5000);
  }, [player]);

  useEffect(() => {
    window.addEventListener("resize", updateBlackBarHeight);
    return () => window.removeEventListener("resize", updateBlackBarHeight);
  }, [updateBlackBarHeight]);

  useEffect(() => {
    if (!canPlay) return;
    updateBlackBarHeight();
  }, [canPlay, updateBlackBarHeight]);

  useEffect(() => {
    if (!video.current || !canPlay || !subReady) return;

    const track = video.current.textTracks[currentTextTrack];

    if (track) {
      track.addEventListener("cuechange", handleCueChange);
    }
  }, [canPlay, currentTextTrack, handleCueChange, subReady, video]);

  return (
    <div className={`videoSubtitles show-${textTrackEnabled && show}`}>
      <p>{currentCue.replace(/<[^>]*>?/gm, "")}</p>
    </div>
  );
}

export default VideoSubtitles;
