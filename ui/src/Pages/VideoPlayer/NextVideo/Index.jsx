import { useEffect } from "react";
import { useDispatch } from "react-redux";
import SelectMediaFile from "../../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../../Modals/SelectMediaFile/Activators/PlayButton";
import { fetchMediaInfo } from "../../../actions/media";

import "./Index.scss";

function NextVideo(props) {
  const { id } = props;
  const dispatch = useDispatch();

  useEffect(() => {
    dispatch(fetchMediaInfo(id));
  }, [dispatch]);

  return (
    <div className="nextVideoOverlay">
      <SelectMediaFile mediaID={id}>
        <SelectMediaFilePlayButton label="Next Episode" hideIcon={true}/>
      </SelectMediaFile>
    </div>
  );
}

export default NextVideo;
