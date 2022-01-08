import { useSelector } from "react-redux";
import SubSwitcher from "./SubSwitcher";
import Settings from "./Settings";

import "./Index.scss";

function VideoMenus() {
  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  return (
    <div className="videoMenus">
      {video.showSubSwitcher && <SubSwitcher />}
      {video.showSettings && <Settings />}
    </div>
  );
}

export default VideoMenus;
