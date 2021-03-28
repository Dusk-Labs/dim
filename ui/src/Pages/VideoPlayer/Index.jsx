import { useCallback, useEffect, useRef, useState } from "react";
import { connect } from "react-redux";
import { MediaPlayer } from "dashjs";

import VideoControls from "./Controls/Index";
import { VideoPlayerContext } from "./Context";
import RingLoad from "../../Components/Load/Ring";
import { clearMediaInfo, fetchMediaInfo } from "../../actions/card";
import ErrorBox from "./ErrorBox";

import "./Index.scss";

// oldOffset logic might still be useful in the future but redundant now
function VideoPlayer(props) {
  const videoPlayer = useRef(null);
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

  const [buffer, setBuffer] = useState(true);
  const [paused, setPaused] = useState(false);
  // const [offset, setOffset] = useState(0);
  const [currentTime, setCurrentTime] = useState(0);
  // const [oldOffset, setOldOffset] = useState(0);
  const [duration, setDuration] = useState(0);

  const {params} = props.match;
  const { fetchMediaInfo, media_info, auth } = props;

  useEffect(() => {
    fetchMediaInfo(auth.token, params.mediaID);
    return () => clearMediaInfo();
  }, [auth.token, fetchMediaInfo, params.mediaID]);

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

    const url = `//${window.host}:8000/api/v1/stream/${params.fileID}/manifest.mpd`;
    const mediaPlayer = MediaPlayer().create();

    mediaPlayer.updateSettings({
      streaming: {
        stableBufferTime: 10,
        bufferToKeep: 10,
        bufferTimeAtTopQuality: 20,
        bufferTimeAtTopQualityLongForm: 20,
        useAppendWindow: true,
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

    setPlayer(mediaPlayer);
  }, [auth.token, params.fileID]);

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
  };

  return (
    <VideoPlayerContext.Provider value={initialValue}>
      <div className="videoPlayer" ref={videoPlayer}>
        <video
          ref={video}
        />
        <div className="overlay">
          {(!error && (manifestLoaded && canPlay)) && <VideoControls/>}
          {(!error & (manifestLoading || !canPlay) || waiting) && <RingLoad/>}
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
  media_info: state.card.media_info
});

const mapActionsToProps = {
  fetchMediaInfo,
  clearMediaInfo
};

export default connect(mapStateToProps, mapActionsToProps)(VideoPlayer);
