import { useParams } from "react-router-dom";
import { useState } from "react";

import { useGetMediaQuery } from "../../api/v1/media";

import Banner from "./Banner";
import MetaContent from "./MetaContent";
import Seasons from "./Seasons";

import MatchMedia from "Pages/Library/MatchMedia/Index";

import "./Index.scss";

function Media() {
  const { id } = useParams();

  const [activeId, setActiveId] = useState(id);

  const { data: media } = useGetMediaQuery(id);

  return (
    <div className="mediaPage">
      <Banner />
      <div className="mediaContent">
        <MatchMedia />
        <div className="meta-content">
          <MetaContent activeId={activeId} />
        </div>
        {media && media.media_type === "tv" && (
          <Seasons setActiveId={setActiveId} />
        )}
      </div>
    </div>
  );
}

export default Media;
