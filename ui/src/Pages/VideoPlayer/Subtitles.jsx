import { useCallback, useEffect, useState, useContext } from "react";
import { calcNewSize } from "../../Helpers/utils";
import { VideoPlayerContext } from "./Context";
import { parseVtt } from "../../Helpers/utils";

import "./Subtitles.scss";

function VideoSubtitles() {
  const { prevSubs, setPrevSubs, subReady, setSubReady, currentCue, setCurrentCue, subtitleTracks, currentSubtitleTrack, player, textTrackEnabled, video, canPlay } = useContext(VideoPlayerContext);

  const [show, setShow] = useState(false);

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

      const cue = e.srcElement.activeCues[0].text
        .replace(/<[^>]*>?/gm, "")
        .split("\n");

      setCurrentCue(cue);
      setShow(true);
    } else {
      setShow(false);
    }
  }, [setCurrentCue]);

  /*
    delete and create new text track as there is no API
    to simply clear text track cues and append new ones
  */
  useEffect(() => {
    if (!video.current || !textTrackEnabled) return;

    console.log("[Subtitles] track changed, removing old text track and creating new one");

    for (const [i, track] of Object.entries(video.current.children)) {
      if (track.kind === "subtitles") {
        video.current.removeChild(video.current.children[i]);
      }
    }

    const newTrack = document.createElement("track");

    newTrack.kind = "subtitles";
    newTrack.track.mode = textTrackEnabled ? "showing" : "hidden";

    video.current.appendChild(newTrack);
  }, [video, currentSubtitleTrack, textTrackEnabled]);

  // clear current cue if track changed
  useEffect(() => {
    setCurrentCue("");
  }, [currentSubtitleTrack, setCurrentCue]);

  useEffect(() => {
    if (subtitleTracks.length === 0 || !textTrackEnabled || !video.current || currentSubtitleTrack === -1 || prevSubs === currentSubtitleTrack) return;

    let prev = 0;

    console.log("[Subtitles] preparing subtitle track", subtitleTracks[currentSubtitleTrack]);

    const intervalID = setInterval(async () => {
      const videoSubTrack = video.current.textTracks[0];

      const req = await fetch(`/api/v1/stream/${subtitleTracks[currentSubtitleTrack].id}/data/stream.vtt`);
      const text = await req.text();

      const diff = text.split(prev).join("");
      const cues = parseVtt(diff);

      if (text && text.length === prev.length) {
        console.log("[Subtitles] subtitles fully loaded");
        clearInterval(intervalID);
        setPrevSubs(currentSubtitleTrack);
      } else {
        prev = text;
      }

      for (let cue of cues) {
        videoSubTrack.addCue(cue);
      }

      setSubReady(true);
    }, 1000);

    return () => {
      console.log("[Subtitles] component unmounted, clearing fetching interval");
      clearInterval(intervalID);
    };
  }, [currentSubtitleTrack, prevSubs, setPrevSubs, setSubReady, subtitleTracks, textTrackEnabled, video]);

  useEffect(() => {
    if (!subReady || !player) return;

    console.log("[Subtitles] ready to show subtitles", currentSubtitleTrack);
  }, [currentSubtitleTrack, player, subReady, textTrackEnabled]);

  useEffect(() => {
    if (video.current) return;
    console.log("[Subtitles] setting player text status to", textTrackEnabled);
    video.current.textTracks[0].mode = textTrackEnabled ? "showing" : "hidden";
  }, [textTrackEnabled, video]);

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

    console.log("[Subtitles] html video tracks", video.current.textTracks);

    const track = video.current.textTracks[0];

    if (track) {
      track.addEventListener("cuechange", handleCueChange);
    }
  }, [canPlay, currentSubtitleTrack, handleCueChange, subReady, video]);

  return (
    <div className={`videoSubtitles show-${textTrackEnabled && show}`}>
      <p>{currentCue}</p>
    </div>
  );
}

export default VideoSubtitles;
