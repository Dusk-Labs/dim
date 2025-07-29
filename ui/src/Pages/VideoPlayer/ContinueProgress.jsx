import { useContext } from "react";
import { useSelector } from "react-redux";
import { skipToken } from "@reduxjs/toolkit/query/react";

import { useGetMediaQuery } from "../../api/v1/media";

import { formatHHMMSS } from "../../Helpers/utils";
import ConfirmationBox from "../../Modals/ConfirmationBox";
import { VideoPlayerContext } from "./Context";

function ContinueProgress() {
  const video = useSelector((store) => store.video);

  const { seekTo } = useContext(VideoPlayerContext);

  const { data: media } = useGetMediaQuery(
    video.mediaID ? video.mediaID : skipToken
  );

  return (
    <ConfirmationBox
      title="Resume watching"
      msg={`You stopped at ${formatHHMMSS(
        media[video.mediaID]?.info.data.progress | 0
      )}`}
      cancelText="Cancel"
      confirmText="Resume"
      action={() => media && seekTo(media.progress)}
    />
  );
}

export default ContinueProgress;
