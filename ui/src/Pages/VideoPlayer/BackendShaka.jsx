import shaka from "shaka-player/dist/shaka-player.compiled";
import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { setManifestState, updateVideo, clearVideoData } from "../../actions/video";

function BackendShaka(props) {
  const { videoRef } = props;

  const dispatch = useDispatch();

  const { manifest, audioTracks, videoTracks, video, auth } = useSelector(store => ({
    auth: store.auth,
    video: store.video,
    manifest: store.video.manifest,
    videoTracks: store.video.tracks.video,
    audioTracks: store.video.tracks.audio
  }));

  window.video = video;
  window.player = videoRef;

  useEffect(() => {
    if (!video.gid || !manifest.virtual.loaded) return;

    console.log("[video] loading manifest");

    dispatch(setManifestState({
      loading: true,
      loaded: false
    }));

    const includes = `${videoTracks.list.map(track => track.id).join(",")},${audioTracks.list.map(track => track.id).join(",")}`;
    const url = `/api/v1/stream/${video.gid}/manifest.mpd?start_num=0&should_kill=false&includes=${includes}`;

    shaka.polyfill.installAll();

    const mediaPlayer = new shaka.Player(videoRef.current);
    mediaPlayer.getNetworkingEngine().registerRequestFilter((_request_type, request) => {
      request.headers["Authorization"] = auth.token;
    });

    mediaPlayer.configure({
      abr: {
        enabled: false
      }
    });

    mediaPlayer.load(url);

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
  }, [videoRef, audioTracks.list, auth.token, dispatch, manifest.virtual.loaded, video.gid, videoTracks.list]);

  return (
    <>
      <video ref={videoRef} autoPlay={true}/>
    </>
  );
}

export default BackendShaka;
