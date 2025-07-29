import { useAppSelector } from "hooks/store";
import SubSwitcher from "./SubSwitcher";
import Settings from "./Settings";

import "./Index.scss";

function VideoMenus() {
  const { video } = useAppSelector((store) => ({
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
