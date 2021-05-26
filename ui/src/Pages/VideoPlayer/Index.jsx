import { useCallback, useEffect, useRef, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { MediaPlayer } from "dashjs";

import { clearMediaInfo, fetchExtraMediaInfo, fetchMediaInfo } from "../../actions/card";
import { VideoPlayerContext } from "./Context";
import VideoEvents from "./Events";

import RingLoad from "../../Components/Load/Ring";
import Menus from "./Menus";
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

  const [ GID, setGID ] = useState();

  const [ videoTracks, setVideoTracks ] = useState([]);
  // eslint-disable-next-line no-unused-vars
  const [ currentVideoTrack, setCurrentVideoTrack ] = useState(0);
  const [ audioTracks, setAudioTracks ] = useState([]);
  // eslint-disable-next-line no-unused-vars
  const [ currentAudioTrack, setCurrentAudioTrack ] = useState(0);
  const [ subtitleTracks, setSubtitleTracks ] = useState([]);
  const [ currentSubtitleTrack, setCurrentSubtitleTrack ] = useState(-1);
  const [ virtualManifestLoaded, setVirtualManifestLoaded ] = useState(false);

  const [subReady, setSubReady] = useState(false);
  const [prevSubs, setPrevSubs] = useState();

  const [manifestLoading, setManifestLoading] = useState(false);
  const [manifestLoaded, setManifestLoaded] = useState(false);
  const [canPlay, setCanPlay] = useState(false);
  const [waiting, setWaiting] = useState(false);
  const [seeking, setSeeking] = useState(false);
  const [fullscreen, setFullscreen] = useState(false);
  const [muted, setMuted] = useState(false);
  const [error, setError] = useState();
  const [paused, setPaused] = useState(false);
  const [textTrackEnabled, setTextTrackEnabled] = useState(false);
  const [episode, setEpisode] = useState();

  const [buffer, setBuffer] = useState(true);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [currentCue, setCurrentCue] = useState("");

  const [ prevSubTrack, setPrevSubTrack ] = useState(0);
  const [ showSubSelection, setShowSubSelection ] = useState(false);

  const [ idleCount, setIdleCount ] = useState(0);

  const { match } = props;
  const { params } = match;
  const { token } = auth;

  useEffect(() => {
    if (GID) return;

    const savedGID = sessionStorage.getItem("videoGID");

    const host = (
      `/api/v1/stream/${params.fileID}/manifest${savedGID ? `?gid=${savedGID}` : ""}`
    );

    (async () => {
      const config = {
        headers: {
          "authorization": token
        }
      };

      const res = await fetch(host, config);
      const payload = await res.json();

      setGID(payload.gid);

      if (!savedGID) {
        sessionStorage.setItem("videoGID", payload.gid);
      }

      const tVideos = payload.tracks.filter(track => track.content_type === "video");
      const tAudios = payload.tracks.filter(track => track.content_type === "audio");
      const tSubtitles = payload.tracks.filter(track => track.content_type === "subtitle");

      setVideoTracks(tVideos);
      setAudioTracks(tAudios);
      setSubtitleTracks(tSubtitles);

      setVirtualManifestLoaded(true);
    })();
  }, [GID, params.fileID, token]);

  useEffect(() => {
    (async () => {
      const config = {
        headers: {
          "authorization": token
        }
      };

      const res = await fetch(`/api/v1/mediafile/${params.fileID}`, config);

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
    if (!mediaID) return;
    dispatch(fetchExtraMediaInfo(mediaID));
    return () => dispatch(clearMediaInfo());
  }, [dispatch, mediaID]);

  useEffect(() => {
    if (!mediaID) return;
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
    if (!GID || !virtualManifestLoaded) return;

    setManifestLoaded(false);
    setManifestLoading(true);

    const includes = `${videoTracks[currentVideoTrack].id},${audioTracks[currentAudioTrack].id}`;
    const url = `/api/v1/stream/${GID}/manifest.mpd?start_num=0&should_kill=false&includes=${includes}`;
    const mediaPlayer = MediaPlayer().create();

    // even with these settings, high bitrate movies fail.
    // The only solution is to have a constant bitrate and cosistent segments.
    // Thus transcoding is the only solution.
    let settings = {
      streaming: {
        stableBufferTime: 20,
        bufferToKeep: 10,
        bufferTimeAtTopQuality: 20,
        bufferTimeAtTopQualityLongForm: 20,
        useAppendWindow: true,
        bufferPruningInterval: 10,
        smallGapLimit: 1000,
      }
    };

    mediaPlayer.updateSettings(settings);

    mediaPlayer.extend("RequestModifier", function () {
      return {
        modifyRequestHeader: function (xhr) {
          xhr.setRequestHeader("Authorization", auth.token);
          return xhr;
        },
        modifyRequestURL: function (url) {
          return url;
        }
      };
    });

    mediaPlayer.initialize(video.current, url, true);

    setPlayer(mediaPlayer);

    return () => {
      mediaPlayer.destroy();

      if (!GID) return;

      (async () => {
        await fetch(`/api/v1/stream/${GID}/state/kill`);
        sessionStorage.clear();
      })();
    };
  }, [GID, audioTracks, auth.token, currentAudioTrack, currentVideoTrack, videoTracks, virtualManifestLoaded]);

  const seekTo = useCallback(async newTime => {
    const newSegment = Math.floor(newTime / 5);

    setCurrentCue("");
    setCurrentTime(newTime);
    setBuffer(0);

    const includes = `${videoTracks[currentVideoTrack].id},${audioTracks[currentAudioTrack].id}`;
    const url = `/api/v1/stream/${GID}/manifest.mpd?start_num=${newSegment}&should_kill=true&includes=${includes}`;

    player.attachSource(url);

    setSeeking(false);
  }, [GID, audioTracks, currentAudioTrack, currentVideoTrack, player, videoTracks]);

  useEffect(() => {
    if (showSubSelection) return;
    setIdleCount(state => state += 1);
  }, [currentTime, showSubSelection]);

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
    textTrackEnabled,
    setTextTrackEnabled,
    overlay: overlay.current,
    seekTo,
    episode,
    videoTracks,
    currentVideoTrack,
    audioTracks,
    currentAudioTrack,
    subtitleTracks,
    currentSubtitleTrack,
    setCurrentSubtitleTrack,
    GID,
    currentCue,
    setCurrentCue,
    showSubSelection,
    setShowSubSelection,
    prevSubTrack,
    setPrevSubTrack,
    subReady,
    setSubReady,
    prevSubs,
    setPrevSubs,
    idleCount,
    setIdleCount,
    setCanPlay,
    setWaiting,
    setPaused,
    setDuration,
    setManifestLoading,
    setManifestLoaded
  };

  return (
    <VideoPlayerContext.Provider value={initialValue}>
      <VideoEvents/>
      <div className="videoPlayer" ref={videoPlayer}>
        <video ref={video}>
          <track
            id="videoSubTrack"
            kind="subtitles"
            default
          />
        </video>
        <VideoSubtitles/>
        <div className="overlay" ref={overlay}>
          {(!error && (manifestLoaded && canPlay && showSubSelection)) && <Menus/>}
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
