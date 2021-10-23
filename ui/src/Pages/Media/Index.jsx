import { useParams } from "react-router-dom";
import { useSelector } from "react-redux";

import Banner from "./Banner";
import MetaContent from "./MetaContent";
import Seasons from "./Seasons";

import "./Index.scss";

function Media() {
  const media = useSelector(store => (
    store.media
  ));

  const { id } = useParams();

  return (
    <div className="mediaPage">
      <Banner/>
      <div className="mediaContent">
        <div>
          <MetaContent/>
        </div>
        {media[id]?.info.data.media_type === "tv" && (
          <Seasons/>
        )}
      </div>
    </div>
  );
}

export default Media;
