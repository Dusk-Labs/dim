import { useCallback, useContext, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import ExpandIcon from "../../../../assets/Icons/Expand";
import CompressIcon from "../../../../assets/Icons/Compress";

import { updateVideo } from "../../../../actions/video";

import { VideoPlayerContext } from "Pages/VideoPlayer/Context";
import { UnfocusableButton } from "Components/unfocusableButton";

function VideoActionFullscreen() {
  const dispatch = useDispatch();

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const { videoPlayer } = useContext(VideoPlayerContext);

  const toggleFullscreen = useCallback(async () => {
    dispatch(
      updateVideo({
        idleCount: 0,
      })
    );

    try {
      if (!document.fullscreenElement) {
        await videoPlayer.current.requestFullscreen();
      } else {
        await document.exitFullscreen();
      }
    } catch (e) {}
  }, [dispatch, videoPlayer]);

  const handleFullscreenChange = useCallback(
    (e) => {
      e.target.blur();
      dispatch(
        updateVideo({
          fullscreen: document.fullscreenElement !== null,
          idleCount: 0,
        })
      );
    },
    [dispatch]
  );

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "f") {
        toggleFullscreen();
      }
    },
    [toggleFullscreen]
  );

  useEffect(() => {
    document.addEventListener("fullscreenchange", handleFullscreenChange);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("fullscreenchange", handleFullscreenChange);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleFullscreenChange, handleKeyDown]);

  if (video.fullscreen)
    return (
      <button onClick={toggleFullscreen} className="fullscreen">
        <CompressIcon />
      </button>
    );
  else
    return (
      <UnfocusableButton onClick={toggleFullscreen} className="fullscreen">
        <ExpandIcon />
      </UnfocusableButton>
    );
}

export default VideoActionFullscreen;
