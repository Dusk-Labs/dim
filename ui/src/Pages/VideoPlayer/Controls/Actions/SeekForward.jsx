import { useCallback, useContext, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import ForwardIcon from "../../../../assets/Icons/Forward";
import { updateVideo } from "../../../../actions/video";
import { VideoPlayerContext } from "../../Context";
import { UnfocusableButton } from "Components/unfocusableButton";

function VideoActionSeekForward() {
  const dispatch = useDispatch();

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const { seekTo } = useContext(VideoPlayerContext);

  const seekForward = useCallback(() => {
    dispatch(
      updateVideo({
        idleCount: 0,
      })
    );

    if (video.currentTime + 15 >= video.duration) {
      seekTo(video.duration);
    } else {
      seekTo(video.currentTime + 15);
    }
  }, [dispatch, seekTo, video.currentTime, video.duration]);

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "ArrowRight") {
        seekForward();
      }
    },
    [seekForward]
  );

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <UnfocusableButton onClick={seekForward} className="forward">
      <ForwardIcon />
    </UnfocusableButton>
  );
}

export default VideoActionSeekForward;
