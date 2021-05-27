import { useCallback, useContext, useEffect } from "react";
import { VideoPlayerContext } from "./Context";
import { MediaPlayer } from "dashjs";

function VideoEvents() {
  const {
    player,
    GID,
    setManifestLoading,
    setManifestLoaded,
    setCanPlay,
    setWaiting,
    setError,
    setPaused,
    setBuffer,
    setCurrentTime,
    setDuration
  } = useContext(VideoPlayerContext);

  const eManifestLoad = useCallback(() => {
    console.log("[VIDEO] manifest loaded");
    setManifestLoading(false);
    setManifestLoaded(true);
  }, [setManifestLoaded, setManifestLoading]);

  const eCanPlay = useCallback(() => {
    console.log("[VIDEO] can play");
    setDuration(Math.round(player.duration()) | 0);
    setCanPlay(true);
    setWaiting(false);
  }, [player, setCanPlay, setDuration, setWaiting]);

  const ePlayBackPaused = useCallback(() => {
    console.log("[VIDEO] paused");
    setPaused(true);
  }, [setPaused]);

  const ePlayBackPlaying = useCallback(() => {
    setPaused(false);
  }, [setPaused]);

  const ePlayBackWaiting = useCallback(() => {
    console.log("[VIDEO] playback waiting");
    setWaiting(true);
  }, [setWaiting]);

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
      const res = await fetch(`/api/v1/stream/${GID}/state/get_stderr`);
      const error = await res.json();

      setError({
        msg: e.error.message,
        errors: error.errors
      });
    })();
  }, [GID, setError]);

  const ePlayBackNotAllowed = useCallback(e => {
    console.log("[VIDEO] playback not allowed");

    if (e.type === "playbackNotAllowed") {
      setPaused(true);
    }
  }, [setPaused]);

  const ePlayBackTimeUpdated = useCallback(e => {
    setCurrentTime(Math.floor(e.time));
    /*
      PLAYBACK_PROGRESS event stops after error occurs
      so using this event from now on to get buffer length
    */
    setBuffer(Math.round(player.getBufferLength()));
  }, [player, setBuffer, setCurrentTime]);

  // other events
  useEffect(() => {
    if (!player) return;

    player.on(MediaPlayer.events.MANIFEST_LOADED, eManifestLoad);
    player.on(MediaPlayer.events.CAN_PLAY, eCanPlay);
    player.on(MediaPlayer.events.ERROR, eError);

    return () => {
      player.off(MediaPlayer.events.MANIFEST_LOADED, eManifestLoad);
      player.off(MediaPlayer.events.CAN_PLAY, eCanPlay);
      player.off(MediaPlayer.events.ERROR, eError);
    };
  }, [eCanPlay, eError, eManifestLoad, player]);

  // video playback
  useEffect(() => {
    if (!player) return;

    player.on(MediaPlayer.events.PLAYBACK_PAUSED, ePlayBackPaused);
    player.on(MediaPlayer.events.PLAYBACK_PLAYING, ePlayBackPlaying);
    player.on(MediaPlayer.events.PLAYBACK_WAITING, ePlayBackWaiting);
    player.on(MediaPlayer.events.PLAYBACK_TIME_UPDATED, ePlayBackTimeUpdated);
    player.on(MediaPlayer.events.PLAYBACK_NOT_ALLOWED, ePlayBackNotAllowed);
    player.on(MediaPlayer.events.PLAYBACK_ENDED, ePlayBackEnded);

    return () => {
      player.off(MediaPlayer.events.PLAYBACK_PAUSED, ePlayBackPaused);
      player.off(MediaPlayer.events.PLAYBACK_PLAYING, ePlayBackPlaying);
      player.off(MediaPlayer.events.PLAYBACK_WAITING, ePlayBackWaiting);
      player.off(MediaPlayer.events.PLAYBACK_TIME_UPDATED, ePlayBackTimeUpdated);
      player.off(MediaPlayer.events.PLAYBACK_NOT_ALLOWED, ePlayBackNotAllowed);
      player.off(MediaPlayer.events.PLAYBACK_ENDED, ePlayBackEnded);
    };
  }, [ePlayBackEnded, ePlayBackNotAllowed, ePlayBackPaused, ePlayBackPlaying, ePlayBackTimeUpdated, ePlayBackWaiting, player]);

  return null;
}

export default VideoEvents;
