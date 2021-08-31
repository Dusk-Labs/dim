import { useContext } from "react";
import { useSelector } from "react-redux";

import { formatHHMMSS } from "../../Helpers/utils";
import ConfirmationBox from "../../Modals/ConfirmationBox";
import { VideoPlayerContext } from "./Context";

function ContinueProgress() {
  const { video, media } = useSelector(store => ({
    video: store.video,
    media: store.media
  }));

  const { seekTo } = useContext(VideoPlayerContext);

  return (
    <ConfirmationBox
      title="Resume watching"
      msg={`You stopped at ${formatHHMMSS(media[video.mediaID]?.info.data.progress | 0)}`}
      cancelText="Cancel"
      confirmText="Resume"
      action={() => seekTo(media[video.mediaID]?.info.data.progress)}
    />
  );
}

export default ContinueProgress;
