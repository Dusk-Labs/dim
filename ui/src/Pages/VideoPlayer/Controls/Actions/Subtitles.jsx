import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import CCIcon from "../../../../assets/Icons/CC";

import { toggleShowSubSwitcher } from "../../../../actions/video";

function VideoActionSubtitles() {
  const dispatch = useDispatch();

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const toggleSubtitles = useCallback(() => {
    dispatch(toggleShowSubSwitcher());
  }, [dispatch]);

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "c") {
        toggleSubtitles();
      }
    },
    [toggleSubtitles]
  );

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <button
      onClick={toggleSubtitles}
      className={`cc trackActive-${video.textTrackEnabled} menuActive-${video.showSubSwitcher}`}
    >
      <CCIcon />
    </button>
  );
}

export default VideoActionSubtitles;
