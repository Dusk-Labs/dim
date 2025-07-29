import { useCallback, useContext, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import BackwardIcon from "../../../../assets/Icons/Backward";
import { updateVideo } from "../../../../actions/video";
import { VideoPlayerContext } from "../../Context";
import { UnfocusableButton } from "Components/unfocusableButton";

function VideoActionSeekBack() {
  const dispatch = useDispatch();

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const { seekTo } = useContext(VideoPlayerContext);

  const seekBackward = useCallback(() => {
    dispatch(
      updateVideo({
        idleCount: 0,
      })
    );

    if (video.currentTime - 15 <= 0) {
      seekTo(0);
    } else {
      seekTo(video.currentTime - 15);
    }
  }, [dispatch, seekTo, video.currentTime]);

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "ArrowLeft") {
        seekBackward();
      }
    },
    [seekBackward]
  );

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <UnfocusableButton onClick={seekBackward} className="backward">
      <BackwardIcon />
    </UnfocusableButton>
  );
}

export default VideoActionSeekBack;
