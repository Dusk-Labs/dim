import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import SelectMediaFile from "../../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../../Modals/SelectMediaFile/Activators/PlayButton";
import { fetchMediaInfo } from "../../../actions/media";

import "./Index.scss";

function NextVideo(props) {
  const { id } = props;
  const dispatch = useDispatch();
  const { video } = useSelector(store => ({
    video: store.video
  }));

  const [visibile, setVisible] = useState(true);

  useEffect(() => {
    setVisible(video.idleCount <= 2);
  }, [video.idleCount, setVisible]);

  useEffect(() => {
    dispatch(fetchMediaInfo(id));
  }, [dispatch]);

  return (
    <div className={`nextVideoOverlay ${visibile}`}>
      <SelectMediaFile mediaID={id}>
        <SelectMediaFilePlayButton label="Next Episode" hideIcon={true}/>
      </SelectMediaFile>
    </div>
  );
}

export default NextVideo;
