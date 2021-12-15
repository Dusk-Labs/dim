import { useCallback, useEffect, useRef } from "react";
import { useParams } from "react-router";
import { useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import { MediaPlayer, Debug } from "dashjs";

import { setTracks, setGID, setManifestState, updateVideo, incIdleCount, clearVideoData } from "../../actions/video";
import { fetchUserSettings } from "../../actions/settings.js";
import { VideoPlayerContext } from "./Context";
import VideoEvents from "./Events";
import VideoMediaData from "./MediaData";

import RingLoad from "../../Components/Load/Ring";
import Menus from "./Menus/Index";
import VideoControls from "./Controls/Index";
import ErrorBox from "./ErrorBox";
import ContinueProgress from "./ContinueProgress";
import VideoSubtitles from "./Subtitles";
import NextVideo from "./NextVideo/Index";

import "./Index.scss";

function VideoPlayer() {
  const params = useParams();
  const dispatch = useDispatch();
  const history = useHistory();

  const { error, manifest, player, audioTracks, videoTracks, video, auth, media, settings } = useSelector(store => ({
    media: store.media,
    auth: store.auth,
    video: store.video,
    player: store.video.player,
    manifest: store.video.manifest,
    videoTracks: store.video.tracks.video,
    audioTracks: store.video.tracks.audio,
    error: store.video.error,
    settings: store.settings
  }));

  const videoPlayer = useRef(null);
  const overlay = useRef(null);
  const videoRef = useRef(null);

  const { token } = auth;

  useEffect(() => {
    if (!video.mediaID) {
      document.title = "Dim - Video Player";
      return;
    }

    if (media[video.mediaID]?.info?.data.name) {
      document.title = `Dim - Playing '${media[video.mediaID].info.data.name}'`;
    }
  }, [media, video.mediaID]);

  // FIXME: Not sure where the best place to do this is, but we need userSettings, but sometimes the user navigates to /play directly so we never fetch userSettings
  useEffect(() => {
    if(settings.userSettings.fetching || settings.userSettings.fetched)
      return;

    dispatch(fetchUserSettings());
  }, [dispatch, settings.userSettings]);

  // If playback finished, redirect to the next video
  useEffect(() => {
    if(!settings?.userSettings?.data?.enable_autoplay) return;

    const currentMedia = media[video.mediaID];
    const nextEpisodeId = currentMedia?.info?.data?.next_episode_id;
    const nextMedia = nextEpisodeId ? media[nextEpisodeId] : null;
    const item = nextMedia?.files?.items[0];

    if(!item) return;

    const ts_diff = video.currentTime - currentMedia?.info?.data?.duration;
    if(video.playback_ended && ts_diff < 10) {
      history.replace(`/play/${item.id}`, { from: history.location.pathname });
    }
  }, [media, video.mediaID, video.currentTime, video.playback_ended, history, settings, settings.userSettings]);

  // Reset GID if play id changes so that this component loads a new video.
  useEffect(() => {
    dispatch(setGID(null));
  }, [params.fileID, dispatch]);

  useEffect(() => {
    if (video.gid) return;

    const host = (
      `/api/v1/stream/${params.fileID}/manifest`
    );

    (async () => {
      const config = {
        headers: {
          "authorization": token
        }
      };

      const res = await fetch(host, config);
      const payload = await res.json();

      dispatch(setGID(payload.gid));

      const tVideos = payload.tracks.filter(track => track.content_type === "video");
      const tAudios = payload.tracks.filter(track => track.content_type === "audio");
      const tSubtitles = payload.tracks.filter(track => track.content_type === "subtitle").filter(track => track.codecs === "vtt");

      dispatch(setTracks({
        video: tVideos,
        audio: tAudios,
        subtitle: tSubtitles
      }));

      dispatch(setManifestState({
        virtual: { loaded: true }
      }));
    })();
  }, [dispatch, params.fileID, token, video.gid]);

  useEffect(() => {
    if (!video.gid || !manifest.virtual.loaded) return;

    console.log("[video] loading manifest");

    dispatch(setManifestState({
      loading: true,
      loaded: false
    }));

    const includes = `${videoTracks.list.map(track => track.id).join(",")},${audioTracks.list.map(track => track.id).join(",")}`;
    const url = `/api/v1/stream/${video.gid}/manifest.mpd?start_num=0&should_kill=false&includes=${includes}`;
    const mediaPlayer = MediaPlayer().create();

    let settings = {
      debug: {
        logLevel: Debug.LOG_LEVEL_DEBUG
      },
      streaming: {
        /* FIXME: Disabling temporarily because the code for this function is unsound
        gaps: {
          enableSeekFix: true
        },
        */
        abr: {
          autoSwitchBitrate: {
            video: false
          }
        }
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

    const getInitialTrack = (trackArr) => {
      const trackList = trackArr[0].type === "video" ? videoTracks.list : audioTracks.list;
      const defaultTrack = trackList.filter(track => track.is_default)[0];
      const initialTracks = trackArr.filter(x => x.id === defaultTrack.set_id);
      console.log(`[${trackArr[0].type}] setting initial track to`, initialTracks);
      return initialTracks;
    };

    mediaPlayer.initialize(videoRef.current, url, true);
    mediaPlayer.setCustomInitialTrackSelectionFunction(getInitialTrack);

    dispatch(updateVideo({
      player: mediaPlayer
    }));

    return () => {
      dispatch(clearVideoData());
      mediaPlayer.destroy();

      if (!video.gid) return;

      (async () => {
        await fetch(`/api/v1/stream/${video.gid}/state/kill`);
        sessionStorage.clear();
      })();
    };
  }, [audioTracks.list, auth.token, dispatch, manifest.virtual.loaded, video.gid, videoTracks.list]);

  const seekTo = useCallback(newTime => {
    player.seek(newTime);

    dispatch(updateVideo({
      seeking: false,
      currentTime: newTime
    }));
  }, [dispatch, player]);

  useEffect(() => {
    if (video.showSubSwitcher) return;
    dispatch(incIdleCount());
  }, [video.currentTime, dispatch, video.showSubSwitcher]);

  const initialValue = {
    videoRef,
    videoPlayer,
    overlay: overlay.current,
    seekTo
  };

  const nextEpisodeId = media[video.mediaID]?.info?.data.next_episode_id;
  const showNextVideoAfter = media[video.mediaID]?.info?.data?.chapters?.credits || 0;

  return (
    <VideoPlayerContext.Provider value={initialValue}>
      <div className="videoPlayer" ref={videoPlayer}>
        <VideoEvents/>
        <VideoMediaData/>
        <video ref={videoRef}/>
        <VideoSubtitles/>
        <div className="overlay" ref={overlay}>
          {(!error && (manifest.loaded && video.canPlay)) && <Menus/>}
          {(!error && (manifest.loaded && video.canPlay) && nextEpisodeId) && <NextVideo id={nextEpisodeId} showAfter={showNextVideoAfter}/>}
          {(!error && (manifest.loaded && video.canPlay)) && <VideoControls/>}
          {(!error & (manifest.loading || !video.canPlay) || video.waiting) && <RingLoad/>}
          {((!error && (manifest.loaded && video.canPlay)) && media[video.mediaID]?.info?.data.progress > 0) && (
            <ContinueProgress/>
          )}
          {error && <ErrorBox/>}
        </div>
      </div>
    </VideoPlayerContext.Provider>
  );
}

export default VideoPlayer;
