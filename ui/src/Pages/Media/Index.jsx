import { useParams } from "react-router-dom";
import { useState } from "react";
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

  const [activeId, setActiveId] = useState(id);

  return (
    <div className="mediaPage">
      <Banner/>
      <div className="mediaContent">
        <div>
          <MetaContent activeId={activeId}/>
        </div>
        {media[id]?.info.data.media_type === "tv" && (
          <Seasons setActiveId={setActiveId}/>
        )}
      </div>
    </div>
  );
}

export default Media;
