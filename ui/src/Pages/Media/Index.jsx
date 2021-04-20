import { useEffect } from "react";
import { useParams } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import { fetchExtraMediaInfo } from "../../actions/card.js";

import Banner from "./Banner";
import MetaContent from "./MetaContent";
import Seasons from "./Seasons.jsx";

import "./Index.scss";

function Media() {
  const dispatch = useDispatch();

  const { media_info, extra_media_info } = useSelector(store => ({
    media_info: store.card.media_info,
    extra_media_info: store.card.extra_media_info
  }));

  const { id } = useParams();

  useEffect(() => {
    dispatch(fetchExtraMediaInfo(id));
  }, [dispatch, id]);

  useEffect(() => {
    const { fetched, error, info } = media_info;

    // FETCH_MEDIA_INFO_OK
    if (fetched && !error) {
      document.title = `Dim - ${info.name}`;
    }
  }, [media_info]);

  return (
    <div className="mediaPage">
      <Banner/>
      <div className="mediaContent">
        <div>
          <MetaContent/>
        </div>
        {extra_media_info.info.seasons && (
          <Seasons/>
        )}
      </div>
    </div>
  );
}

export default Media;
