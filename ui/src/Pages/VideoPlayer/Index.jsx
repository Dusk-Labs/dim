import { useCallback, useEffect, useRef, useState } from "react";
import { connect } from "react-redux";
import { MediaPlayer } from "dashjs";

import VideoControls from "./Controls/Index";
import { VideoPlayerContext } from "./Context";
import RingLoad from "../../Components/Load/Ring";
import { clearMediaInfo, fetchExtraMediaInfo, fetchMediaInfo } from "../../actions/card";
import ErrorBox from "./ErrorBox";
import ContinueProgress from "./ContinueProgress";

import "./Index.scss";

// oldOffset logic might still be useful in the future but redundant now
function VideoPlayer(props) {
  const videoPlayer = useRef(null);
  const overlay = useRef(null);
  const video = useRef(null);

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
  const [episode, setEpisode] = useState();

  const [buffer, setBuffer] = useState(true);
  const [paused, setPaused] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);

  const { clearMediaInfo, fetchExtraMediaInfo, fetchMediaInfo, media_info, auth, match } = props;
  const { params } = match;

  useEffect(() => {
    if (props.extra_media_info.info.seasons) {
      const { seasons } = props.extra_media_info.info;

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
        console.log(episode)
      }
    }
  }, [params.fileID, props.extra_media_info.info]);

  useEffect(() => {
    fetchExtraMediaInfo(auth.token, params.mediaID);
    return () => clearMediaInfo()
  }, [auth.token, clearMediaInfo, fetchExtraMediaInfo, params.mediaID]);

  useEffect(() => {
    fetchMediaInfo(auth.token, params.mediaID);
    return () => clearMediaInfo();
  }, [auth.token, clearMediaInfo, fetchMediaInfo, params.mediaID]);

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

  const eError = useCallback(e => {
    setError(e.error)
  }, []);

  const ePlayBackNotAllowed = useCallback(e => {
    if (e.type === "playbackNotAllowed") {
      setPaused(true);
    }
  }, []);

  /*
    Seeking first time to 100s results in video.time starting from 0s
    Seeking second time to 200s results in video.time taking the old seek position starting from 100s
    OldOffset undos that and sets it back to 0s for consistency and to keep track of seekbar position accurately
  */
  const ePlayBackTimeUpdated = useCallback(e => {
    // setCurrentTime(Math.floor(offset + (e.time - oldOffset)));
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
    player.on(MediaPlayer.events.ERROR, eError);

    return () => {
      player.off(MediaPlayer.events.MANIFEST_LOADED, eManifestLoad);
      player.off(MediaPlayer.events.CAN_PLAY, eCanPlay);
      player.off(MediaPlayer.events.PLAYBACK_PAUSED, ePlayBackPaused);
      player.off(MediaPlayer.events.PLAYBACK_PLAYING, ePlayBackPlaying);
      player.off(MediaPlayer.events.PLAYBACK_WAITING, ePlayBackWaiting);
      player.off(MediaPlayer.events.PLAYBACK_TIME_UPDATED, ePlayBackTimeUpdated);
      player.off(MediaPlayer.events.PLAYBACK_NOT_ALLOWED, ePlayBackNotAllowed);
      player.off(MediaPlayer.events.ERROR, eError);
    }
  }, [eCanPlay, eError, eManifestLoad, ePlayBackNotAllowed, ePlayBackPaused, ePlayBackPlaying, ePlayBackTimeUpdated, ePlayBackWaiting, player])

  const initialValue = {
    player,
    mediaInfo: props.media_info.info,
    mediaID: params.mediaID,
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
    videoUUID,
    overlay: overlay.current,
    seekTo,
    episode
  };

  return (
    <VideoPlayerContext.Provider value={initialValue}>
      <div className="videoPlayer" ref={videoPlayer}>
        <video ref={video}/>
        <div className="overlay" ref={overlay}>
          {(!error && (manifestLoaded && canPlay)) && <VideoControls/>}
          {(!error & (manifestLoading || !canPlay) || waiting) && <RingLoad/>}
          {((!error && (manifestLoaded && canPlay)) && props.extra_media_info.info.progress > 0) && (
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

const mapStateToProps = (state) => ({
  auth: state.auth,
  media_info: state.card.media_info,
  extra_media_info: state.card.extra_media_info
});

const mapActionsToProps = {
  fetchMediaInfo,
  fetchExtraMediaInfo,
  clearMediaInfo
};

export default connect(mapStateToProps, mapActionsToProps)(VideoPlayer);
