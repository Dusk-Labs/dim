import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { MediaPlayer } from "dashjs";

import { setManifestState, updateTrack, updateVideo } from "../../actions/video";

function VideoEvents() {
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

    window.video = video;
    // we need to do all this shit so that the UI selects the correct tracks.
    const videoQualityIndex = player.getQualityFor("video");
    const videoQuality = player.getBitrateInfoListFor("video")[videoQualityIndex];

    const playerVideoTrackIdx = video.tracks.video.list.filter(track => track.bandwidth === videoQuality.bitrate && parseInt(track.height) === videoQuality.height);

    dispatch(updateTrack("video", {
      current: video.tracks.video.list.indexOf(playerVideoTrackIdx[0])
    }));

    dispatch(updateVideo({
      canPlay: true,
      waiting: false,
      duration: Math.round(player.duration()) | 0
    }));
  }, [dispatch, player, video]);

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

  const ePlayBackNotAllowed = useCallback(e => {
    console.log("[VIDEO] playback not allowed");

    if (e.type === "playbackNotAllowed") {
      dispatch(updateVideo({
        paused: true
      }));
    }
  }, [dispatch]);

  /*
    PLAYBACK_PROGRESS event stops after error occurs
    so using this event from now on to get buffer length
  */
  const ePlayBackTimeUpdated = useCallback(e => {
    /*
      on some browsers (*cough*, chrome) current
      time gets reset back to 0 on seek
    */
    let newTime = Math.floor(e.time);

    if (newTime < video.prevSeekTo) {
      newTime += video.prevSeekTo - newTime;
    }

    dispatch(updateVideo({
      currentTime: newTime,
      buffer: Math.round(player.getBufferLength()),
      waiting: false
    }));
  }, [dispatch, player, video.prevSeekTo]);

  const eQualityChange = useCallback(e => {
    console.log("[video] quality changing ", e);

    if (e.mediaType !== "video") return;

    const tracks = e.mediaType === "video" ? video.tracks.video.list : video.tracks.audio.list;

    // here we gotta basically do the opposite of what we do in Settings.jsx
    const newTrack = player.getBitrateInfoListFor(e.mediaType)[e.newQuality];
    const realTrack = tracks.filter(track => track.bandwidth === newTrack.bitrate && parseInt(track.height) === newTrack.height)[0];

    dispatch(updateTrack(e.mediaType, {
      current: tracks.indexOf(realTrack)
    }));
  }, [dispatch, player, video]);

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
    player.on(MediaPlayer.events.QUALITY_CHANGE_REQUESTED, eQualityChange);

    return () => {
      player.off(MediaPlayer.events.PLAYBACK_PAUSED, ePlayBackPaused);
      player.off(MediaPlayer.events.PLAYBACK_PLAYING, ePlayBackPlaying);
      player.off(MediaPlayer.events.PLAYBACK_WAITING, ePlayBackWaiting);
      player.off(MediaPlayer.events.PLAYBACK_TIME_UPDATED, ePlayBackTimeUpdated);
      player.off(MediaPlayer.events.PLAYBACK_NOT_ALLOWED, ePlayBackNotAllowed);
      player.off(MediaPlayer.events.PLAYBACK_ENDED, ePlayBackEnded);
      player.off(MediaPlayer.events.QUALITY_CHANGE_REQUESTED, eQualityChange);
    };
  }, [ePlayBackEnded, ePlayBackNotAllowed, ePlayBackPaused, ePlayBackPlaying, ePlayBackTimeUpdated, ePlayBackWaiting, eQualityChange, player]);

  return null;
}

export default VideoEvents;
