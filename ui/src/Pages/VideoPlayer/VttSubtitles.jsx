import { useCallback, useEffect, useState, useContext } from "react";
import { useDispatch, useSelector } from "react-redux";

import { calcNewSize } from "../../Helpers/utils";
import { VideoPlayerContext } from "./Context";
import { parseVtt } from "../../Helpers/utils";
import { updateTrack, updateVideo } from "../../actions/video";

import "./Subtitles.scss";

function VideoSubtitles() {
  const dispatch = useDispatch();

  const { video, current, tracks, ready } = useSelector((store) => ({
    video: store.video,
    current: store.video.tracks.subtitle.current,
    tracks: store.video.tracks.subtitle.list,
    ready: store.video.tracks.subtitle.ready,
  }));
  const isVtt = tracks[current]?.chunk_path?.endsWith("vtt");

  const { videoRef } = useContext(VideoPlayerContext);

  const [show, setShow] = useState(false);

  // relative to window width
  const updateBlackBarHeight = useCallback(() => {
    const videoHeight = calcNewSize(
      videoRef.current.videoWidth,
      videoRef.current.videoHeight,
      window.innerWidth
    );

    const blackBarHeight = (window.innerHeight - videoHeight) / 2;

    if (blackBarHeight > 100) {
      document.documentElement.style.setProperty(
        "--blackBarHeight",
        `${blackBarHeight}px`
      );
    }
  }, [videoRef]);

  const handleCueChange = useCallback(
    (e) => {
      if (e.srcElement.activeCues.length > 0) {
        const cue = Object.entries(e.srcElement.activeCues).map(([_, x]) =>
          x.text.replace(/<[^>]*>?/gm, "").split("\n")
        );

        dispatch(
          updateVideo({
            currentCue: cue,
          })
        );
        setShow(true);
      } else {
        setShow(false);
      }
    },
    [dispatch]
  );

  /*
    delete and create new text track as there is no API
    to simply clear text track cues and append new ones
  */
  useEffect(() => {
    if (!videoRef.current || !video.textTrackEnabled || !isVtt) return;

    console.log("[Subtitles] track changed");

    for (const [i, track] of Object.entries(videoRef.current.children)) {
      if (track.kind === "subtitles") {
        console.log("[Subtitles] removed old text track");
        videoRef.current.removeChild(videoRef.current.children[i]);
      }
    }

    const newTrack = document.createElement("track");

    newTrack.kind = "subtitles";
    newTrack.track.mode = video.textTrackEnabled ? "showing" : "hidden";

    console.log("[Subtitles] created and appended new track");
    videoRef.current.appendChild(newTrack);
  }, [video.textTrackEnabled, videoRef, current, isVtt]);

  // clear current cue if track changed
  useEffect(() => {
    dispatch(
      updateVideo({
        currentCue: [],
      })
    );
  }, [dispatch, current]);

  useEffect(() => {
    if (
      !isVtt ||
      !video.textTrackEnabled ||
      !videoRef.current ||
      current === -1 ||
      video.prevSubs === current
    )
      return;

    let prev = 0;

    console.log("[Subtitles] preparing subtitle track", tracks[current]);

    const intervalID = setInterval(async () => {
      const videoSubTrack = videoRef.current.textTracks[0];

      const req = await fetch(`/api/v1/stream/${tracks[current].chunk_path}`);
      const text = await req.text();

      const diff = text.split(prev).join("");
      const cues = parseVtt(diff);

      if (text && text.length === prev.length) {
        console.log("[Subtitles] subtitles fully loaded");

        clearInterval(intervalID);

        dispatch(
          updateVideo({
            prevSubs: current,
          })
        );
      } else {
        console.log("[Subtitles] fetching again in 1 second", text.length);
        prev = text;
      }

      for (let cue of cues) {
        videoSubTrack.addCue(cue);
      }

      dispatch(
        updateTrack("subtitle", {
          ready: true,
        })
      );
    }, 1000);

    return () => {
      console.log(
        "[Subtitles] component unmounted, clearing fetching interval"
      );
      clearInterval(intervalID);
    };
  }, [
    current,
    dispatch,
    tracks,
    video.prevSubs,
    video.textTrackEnabled,
    videoRef,
    isVtt,
  ]);

  useEffect(() => {
    if (videoRef.current || !isVtt) return;
    console.log(
      "[Subtitles] setting player text status to",
      video.textTrackEnabled
    );
    videoRef.current.textTracks[0].mode = video.textTrackEnabled
      ? "showing"
      : "hidden";
  }, [video.textTrackEnabled, videoRef, isVtt]);

  useEffect(() => {
    window.addEventListener("resize", updateBlackBarHeight);
    return () => window.removeEventListener("resize", updateBlackBarHeight);
  }, [updateBlackBarHeight]);

  useEffect(() => {
    if (!video.canPlay) return;
    updateBlackBarHeight();
  }, [video.canPlay, updateBlackBarHeight]);

  useEffect(() => {
    if (!videoRef.current || !video.canPlay || !ready || !isVtt) return;

    console.log("[Subtitles] html video tracks", videoRef.current.textTracks);

    const track = videoRef.current.textTracks[0];

    if (track) {
      track.addEventListener("cuechange", handleCueChange);
    }
  }, [handleCueChange, ready, video.canPlay, videoRef, isVtt]);

  return (
    <div className={`videoSubtitles show-${video.textTrackEnabled && show}`}>
      {video.currentCue.map((x, key) => (
        <p key={key}>{x}</p>
      ))}
    </div>
  );
}

export default VideoSubtitles;
