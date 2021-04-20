import { useContext } from "react";
import { useSelector } from "react-redux";

import { formatHHMMSS } from "../../Helpers/utils";
import ConfirmationBox from "../../Modals/ConfirmationBox";
import { VideoPlayerContext } from "./Context";

function ContinueProgress() {
  const extra_media_info = useSelector(store => (
    store.card.extra_media_info
  ));

  const { seekTo } = useContext(VideoPlayerContext);

  return (
    <ConfirmationBox
      title="Resume watching"
      msg={`You stopped at ${formatHHMMSS(extra_media_info?.info?.progress | 0)}`}
      cancelText="Cancel"
      confirmText="Resume"
      action={() => seekTo(extra_media_info?.info?.progress)}
    />
  );
}

export default ContinueProgress;
