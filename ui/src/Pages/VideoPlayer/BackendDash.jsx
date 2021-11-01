import { MediaPlayer, Debug } from "dashjs";
import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { setManifestState, updateVideo, clearVideoData } from "../../actions/video";

function BackendDash(props) {
  const { videoRef } = props;

  const dispatch = useDispatch();

  const { manifest, audioTracks, videoTracks, video, auth } = useSelector(store => ({
    auth: store.auth,
    video: store.video,
    manifest: store.video.manifest,
    videoTracks: store.video.tracks.video,
    audioTracks: store.video.tracks.audio
  }));

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

    // even with these settings, high bitrate movies fail.
    // The only solution is to have a constant bitrate and cosistent segments.
    // Thus transcoding is the only solution.
    let settings = {
      debug: {
        logLevel: Debug.LOG_LEVEL_DEBUG
      },
      streaming: {
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
  }, [videoRef, audioTracks.list, auth.token, dispatch, manifest.virtual.loaded, video.gid, videoTracks.list]);

  return (
    <>
      <video ref={videoRef}/>
    </>
  );
}

export default BackendDash;
