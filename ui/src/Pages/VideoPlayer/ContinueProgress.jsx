import { useContext } from "react";
import { connect } from "react-redux";
import { clearMediaInfo, fetchExtraMediaInfo } from "../../actions/card";
import { formatHHMMSS } from "../../Helpers/utils";
import ConfirmationBox from "../../Modals/ConfirmationBox";
import { VideoPlayerContext } from "./Context";

function ContinueProgress(props) {
  const { seekTo } = useContext(VideoPlayerContext);

  const { extra_media_info } = props;

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

const mapStateToProps = (state) => ({
  auth: state.auth,
  extra_media_info: state.card.extra_media_info
});

const mapActionsToProps = {
  fetchExtraMediaInfo,
  clearMediaInfo
};

export default connect(mapStateToProps, mapActionsToProps)(ContinueProgress);
