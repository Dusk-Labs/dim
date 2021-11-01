import { useCallback, useEffect, useRef } from "react";
import { useParams } from "react-router";
import { useDispatch, useSelector } from "react-redux";

import { setTracks, setGID, setManifestState, updateVideo, incIdleCount } from "../../actions/video";
import { VideoPlayerContext } from "./Context";
import VideoEvents from "./Events";
import VideoMediaData from "./MediaData";

import RingLoad from "../../Components/Load/Ring";
import Menus from "./Menus/Index";
import VideoControls from "./Controls/Index";
import ErrorBox from "./ErrorBox";
import ContinueProgress from "./ContinueProgress";
import VideoSubtitles from "./Subtitles";

import BackendDash from "./BackendDash";
import BackendShaka from "./BackendShaka";

import ShakaEvents from "./ShakaEvents";

import "./Index.scss";

function VideoPlayer() {
  const params = useParams();
  const dispatch = useDispatch();

  const { error, manifest, player, video, auth, media } = useSelector(store => ({
    media: store.media,
    auth: store.auth,
    video: store.video,
    player: store.video.player,
    manifest: store.video.manifest,
    error: store.video.error
  }));

  const videoPlayer = useRef(null);
  const overlay = useRef(null);
  const videoRef = useRef(null);

  const { token } = auth;

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
    if (!video.mediaID) {
      document.title = "Dim - Video Player";
      return;
    }

    if (media[video.mediaID]?.info?.data.name) {
      document.title = `Dim - Playing '${media[video.mediaID].info.data.name}'`;
    }
  }, [media, video.mediaID]);

  const seekTo = useCallback(newTime => {
    //player.seek(newTime);
    player.getMediaElement().currentTime = newTime;

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
  //<VideoEvents/>

  return (
    <VideoPlayerContext.Provider value={initialValue}>
      <div className="videoPlayer" ref={videoPlayer}>
        <ShakaEvents videoRef={videoRef}/>
        <VideoMediaData/>
        <BackendShaka videoRef={videoRef}/>
        <VideoSubtitles/>
        <div className="overlay" ref={overlay}>
          {(!error && (manifest.loaded && video.canPlay)) && <Menus/>}
          {(!error && (manifest.loaded && video.canPlay)) && <VideoControls/>}
          {(!error & (manifest.loading || !video.canPlay) || video.waiting) && <RingLoad/>}
          {((!error && (manifest.loaded && video.canPlay)) && media[video.mediaID]?.info.data.progress > 0) && (
            <ContinueProgress/>
          )}
          {error && <ErrorBox/>}
        </div>
      </div>
    </VideoPlayerContext.Provider>
  );
}

export default VideoPlayer;
