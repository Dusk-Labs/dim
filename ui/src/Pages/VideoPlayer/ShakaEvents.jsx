import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import shaka from "shaka-player/dist/shaka-player.compiled";

import { setManifestState, updateTrack, updateVideo } from "../../actions/video";

function ShakaEvents() {
  const dispatch = useDispatch();

  const { video, player } = useSelector(store => ({
    video: store.video,
    player: store.video.player
  }));

  const eManifestLoad = useCallback(() => {
    console.log("[VIDEO] manifest loaded");

    dispatch(setManifestState({
      loading: false,
      loaded: true
    }));
  }, [dispatch]);

  const eCanPlay = useCallback(() => {
    console.log("[VIDEO] can play");
    console.log(player.getMediaElement().duration);

    dispatch(updateVideo({
      canPlay: true,
      waiting: false,
      duration: Math.round(player.getMediaElement().duration) | 0
    }));
  }, [dispatch, player]);

  const ePlayBackPaused = useCallback(() => {
    console.log("[VIDEO] paused");

    dispatch(updateVideo({
      paused: true
    }));
  }, [dispatch]);

  const ePlayBackPlaying = useCallback(() => {
    dispatch(updateVideo({
      paused: false
    }));
  }, [dispatch]);

  const ePlayBackWaiting = useCallback(() => {
    console.log("[VIDEO] playback waiting");

    dispatch(updateVideo({
      waiting: true
    }));
  }, [dispatch]);

  const ePlayBackEnded = useCallback(e => {
    console.log("[VIDEO] playback ended", e);
  }, []);

  const eError = useCallback(e => {
    // segment not available
    if (e.error.code === 27) {
      console.log("[VIDEO] segment not available", e.error.message);
      return;
    }

    (async () => {
      console.log("[VIDEO] fetching stderr");
      const res = await fetch(`/api/v1/stream/${video.gid}/state/get_stderr`);
      const error = await res.json();

      dispatch(updateVideo({
        error: {
          msg: e.error.message,
          errors: error.errors
        }
      }));
    })();
  }, [dispatch, video.gid]);

  const ePlayBackTimeUpdated = useCallback(e => {
    // FIXME: Math.round(player.getBufferLength())
    const buffer = player.getBufferedInfo().total[0].end;

    dispatch(updateVideo({
      currentTime: e.time || player.getMediaElement().currentTime,
      waiting: false,
      buffer
    }));
  }, [dispatch, player]);

  // video playback
  useEffect(() => {
    if (!player) return;

    player.addEventListener("manifestparsed", eManifestLoad);
    player.addEventListener("loaded", eCanPlay);
    player.addEventListener("error", eError);
    player.getMediaElement().addEventListener("pause", ePlayBackPaused);
    player.getMediaElement().addEventListener("playing", ePlayBackPlaying);
    player.addEventListener("waiting", ePlayBackWaiting);
    player.getMediaElement().addEventListener("timeupdate", ePlayBackTimeUpdated);
    player.addEventListener("ended", ePlayBackEnded);

    return () => {
      player.removeEventListener("manifestparsed", eManifestLoad);
      player.removeEventListener("loaded", eCanPlay);
      player.removeEventListener("error", eError);
      player.removeEventListener("pause", ePlayBackPaused);
      player.removeEventListener("playing", ePlayBackPlaying);
      player.removeEventListener("waiting", ePlayBackWaiting);
      player.removeEventListener("timeupdate", ePlayBackTimeUpdated);
      player.removeEventListener("ended", ePlayBackEnded);
    };
  }, [eManifestLoad, eCanPlay, eError, ePlayBackEnded, ePlayBackPaused, ePlayBackPlaying, ePlayBackTimeUpdated, ePlayBackWaiting, player]);

  return null;
}

export default ShakaEvents;
