import { useCallback, useEffect, useRef, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { MediaPlayer } from "dashjs";

import { clearMediaInfo, fetchExtraMediaInfo, fetchMediaInfo } from "../../actions/card";
import { VideoPlayerContext } from "./Context";
import RingLoad from "../../Components/Load/Ring";
import VideoControls from "./Controls/Index";
import ErrorBox from "./ErrorBox";
import ContinueProgress from "./ContinueProgress";
import VideoSubtitles from "./Subtitles";

import "./Index.scss";

/*
  logic for media name and other metadata is in place,
  awaiting info to be returned by API - hidden until then.
*/

// TODO: useReducer the shit out of this shit.
function VideoPlayer(props) {
  const dispatch = useDispatch();

  const { auth, media_info, extra_media_info } = useSelector(store => ({
    auth: store.auth,
    media_info: store.card.media_info,
    extra_media_info: store.card.extra_media_info
  }));

  const videoPlayer = useRef(null);
  const overlay = useRef(null);
  const video = useRef(null);

  const [mediaID, setMediaID] = useState();

  const [player, setPlayer] = useState();

  const [manifestLoading, setManifestLoading] = useState(false);
  const [manifestLoaded, setManifestLoaded] = useState(false);
  const [canPlay, setCanPlay] = useState(false);
  const [waiting, setWaiting] = useState(false);
  const [seeking, setSeeking] = useState(false);
  const [fullscreen, setFullscreen] = useState(false);
  const [muted, setMuted] = useState(false);
  const [error, setError] = useState();
  const [videoUUID, setVideoUUID] = useState();
  const [paused, setPaused] = useState(false);
  const [currentTextTrack, setCurrentTextTrack] = useState(0);
  const [textTrackEnabled, setTextTrackEnabled] = useState(false);
  const [episode, setEpisode] = useState();

  const [buffer, setBuffer] = useState(true);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);

  const { match } = props;
  const { params } = match;
  const { token } = auth;

  useEffect(() => {
    (async () => {
      const config = {
        headers: {
            "authorization": token
        }
      };

      const res = await fetch(`//${window.host}:8000/api/v1/mediafile/${params.fileID}`, config);

      if (res.status !== 200) {
        return;
      }

      const payload = await res.json();

      setMediaID(payload.media_id);
    })();
  }, [params.fileID, token]);

  useEffect(() => {
    if (extra_media_info.info.seasons) {
      const { seasons } = extra_media_info.info;

      let episode;

      for (const season of seasons) {
        const found = season.episodes.filter(ep => {
          return ep.versions.filter(version => version.id === parseInt(params.fileID)).length === 1;
        });

        if (found.length > 0) {
          episode = {
            ...found[0],
            season: season.season_number
          };

          break;
        }
      }

      if (episode) {
        setEpisode(episode);
      }
    }
  }, [extra_media_info.info, params.fileID]);

  useEffect(() => {
    dispatch(fetchExtraMediaInfo(mediaID));
    return () => dispatch(clearMediaInfo())
  }, [dispatch, mediaID]);

  useEffect(() => {
    dispatch(fetchMediaInfo(mediaID));
    return () => dispatch(clearMediaInfo());
  }, [dispatch, mediaID]);

  useEffect(() => {
    document.title = "Dim - Video Player";

    if (media_info.info.name) {
      document.title = `Dim - Playing '${media_info.info.name}'`;
    }
  }, [media_info.info.name]);

  useEffect(() => {
    if (!params.fileID) return;

    setManifestLoaded(false);
    setManifestLoading(true);

    const existingUUID = sessionStorage.getItem("videoUUID");

    let uuid;

    if (existingUUID) {
      uuid = existingUUID;
    } else {
      uuid = "xxxxxxxxxxxxxxxx".replace(/[xy]/g, () => Math.round(Math.random() * 8));
      sessionStorage.setItem("videoUUID", uuid);
    }

    const url = `//${window.host}:8000/api/v1/stream/${params.fileID}/manifest.mpd?gid=${uuid}`;
    const mediaPlayer = MediaPlayer().create();

    mediaPlayer.updateSettings({
      streaming: {
        stableBufferTime: 10,
        bufferToKeep: 10,
        bufferTimeAtTopQuality: 20,
        bufferTimeAtTopQualityLongForm: 20,
        useAppendWindowEnd: false,
        bufferPruningInterval: 10,
      }
    });

    mediaPlayer.extend("RequestModifier", function () {
      return {
        modifyRequestHeader: function (xhr) {
          xhr.setRequestHeader("Authorization", auth.token);
          return xhr;
        },
        modifyRequestURL: function (url) {
          return url;
        }
      }
    });

    mediaPlayer.initialize(video.current, url, true);
    mediaPlayer.enableForcedTextStreaming(true);

    setPlayer(mediaPlayer);
    setVideoUUID(uuid);

    return () => {
      mediaPlayer.destroy();

      const uuid = sessionStorage.getItem("videoUUID");
      if (!uuid) return;

      (async () => {
        await fetch(`//${window.host}:8000/api/v1/stream/${uuid}/state/kill`);
        sessionStorage.clear();
      })();
    }
  }, [auth.token, params.fileID]);

  const seekTo = useCallback(async newTime => {
    const newSegment = Math.floor(newTime / 5);

    setCurrentTime(newTime);
    setBuffer(0);

    player.attachSource(`//${window.host}:8000/api/v1/stream/${params.fileID}/manifest.mpd?start_num=${newSegment}&gid=${videoUUID}`);

    setSeeking(false);
  }, [params.fileID, player, videoUUID]);

  const eManifestLoad = useCallback(() => {
    setManifestLoading(false);
    setManifestLoaded(true);
  }, []);

  const eCanPlay = useCallback(() => {
    setDuration(Math.round(player.duration()) | 0);
    setCanPlay(true);
    setWaiting(false);
  }, [player]);

  const ePlayBackPaused = useCallback(() => {
    setPaused(true);
  }, []);

  const ePlayBackPlaying = useCallback(() => {
    setPaused(false);
  }, []);

  const ePlayBackWaiting = useCallback(e => {
    setWaiting(true);
  }, []);

  const ePlayBackEnded = useCallback(e => {
    console.log("PLAYBACK ENDED", e);
  }, []);

  const eError = useCallback(e => {
    // segment not available
    if (e.error.code === 27) {
      console.log("segment not available", e.error.message)
      return;
    }

    (async () => {
      const res = await fetch(`//${window.host}:8000/api/v1/stream/${videoUUID}/state/get_stderr`);
      const error = await res.json();

      setError({
        msg: e.error.message,
        errors: error.errors
      });
    })();
  }, [videoUUID]);

  const ePlayBackNotAllowed = useCallback(e => {
    if (e.type === "playbackNotAllowed") {
      setPaused(true);
    }
  }, []);

  const ePlayBackTimeUpdated = useCallback(e => {
    setCurrentTime(Math.floor(e.time));
    /*
      PLAYBACK_PROGRESS event stops after error occurs
      so using this event from now on to get buffer length
    */
    setBuffer(Math.round(player.getBufferLength()));
  }, [player]);

  // video events
  useEffect(() => {
    if (!player) return;

    player.on(MediaPlayer.events.MANIFEST_LOADED, eManifestLoad);
    player.on(MediaPlayer.events.CAN_PLAY, eCanPlay);
    player.on(MediaPlayer.events.PLAYBACK_PAUSED, ePlayBackPaused);
    player.on(MediaPlayer.events.PLAYBACK_PLAYING, ePlayBackPlaying);
    player.on(MediaPlayer.events.PLAYBACK_WAITING, ePlayBackWaiting);
    player.on(MediaPlayer.events.PLAYBACK_TIME_UPDATED, ePlayBackTimeUpdated);
    player.on(MediaPlayer.events.PLAYBACK_NOT_ALLOWED, ePlayBackNotAllowed);
    player.on(MediaPlayer.events.PLAYBACK_ENDED, ePlayBackEnded);
    player.on(MediaPlayer.events.ERROR, eError);

    return () => {
      player.off(MediaPlayer.events.MANIFEST_LOADED, eManifestLoad);
      player.off(MediaPlayer.events.CAN_PLAY, eCanPlay);
      player.off(MediaPlayer.events.PLAYBACK_PAUSED, ePlayBackPaused);
      player.off(MediaPlayer.events.PLAYBACK_PLAYING, ePlayBackPlaying);
      player.off(MediaPlayer.events.PLAYBACK_WAITING, ePlayBackWaiting);
      player.off(MediaPlayer.events.PLAYBACK_TIME_UPDATED, ePlayBackTimeUpdated);
      player.off(MediaPlayer.events.PLAYBACK_NOT_ALLOWED, ePlayBackNotAllowed);
      player.off(MediaPlayer.events.PLAYBACK_ENDED, ePlayBackEnded);
      player.off(MediaPlayer.events.ERROR, eError);
    }
  }, [eCanPlay, eError, eManifestLoad, ePlayBackEnded, ePlayBackNotAllowed, ePlayBackPaused, ePlayBackPlaying, ePlayBackTimeUpdated, ePlayBackWaiting, player])

  const initialValue = {
    player,
    mediaInfo: media_info.info,
    mediaID,
    fileID: params.fileID,
    video,
    videoPlayer,
    fullscreen,
    setFullscreen,
    seeking,
    muted,
    setMuted,
    setSeeking,
    setCurrentTime,
    currentTime,
    duration,
    setPlayer,
    setBuffer,
    buffer,
    paused,
    canPlay,
    currentTextTrack,
    setCurrentTextTrack,
    textTrackEnabled,
    setTextTrackEnabled,
    videoUUID,
    overlay: overlay.current,
    seekTo,
    episode
  };

  return (
    <VideoPlayerContext.Provider value={initialValue}>
      <div className="videoPlayer" ref={videoPlayer}>
        <video ref={video}/>
        <VideoSubtitles/>
        <div className="overlay" ref={overlay}>
          {(!error && (manifestLoaded && canPlay)) && <VideoControls/>}
          {(!error & (manifestLoading || !canPlay) || waiting) && <RingLoad/>}
          {((!error && (manifestLoaded && canPlay)) && extra_media_info.info.progress > 0) && (
            <ContinueProgress/>
          )}
          {error && (
            <ErrorBox error={error} setError={setError} currentTime={currentTime}/>
          )}
        </div>
      </div>
    </VideoPlayerContext.Provider>
  );
}

export default VideoPlayer;
