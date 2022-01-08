import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import SettingsIcon from "../../../../assets/Icons/Settings";

import { toggleShowSettings } from "../../../../actions/video";

function VideoActionSettings() {
  const dispatch = useDispatch();

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const toggleSettings = useCallback(() => {
    dispatch(toggleShowSettings());
  }, [dispatch]);

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "c") {
        toggleSettings();
      }
    },
    [toggleSettings]
  );

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <button
      onClick={toggleSettings}
      className={`settings menuActive-${video.showSettings}`}
    >
      <SettingsIcon />
    </button>
  );
}

export default VideoActionSettings;
