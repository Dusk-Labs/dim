import { useEffect } from "react";
import { connect } from "react-redux";
import { useParams } from "react-router-dom";

import {
  fetchExtraMediaInfo,
  fetchMediaSeasons,
  fetchMediaSeasonEpisodes
} from "../../actions/card.js";

import Banner from "./Banner";
import MetaContent from "./MetaContent";
import Seasons from "./Seasons.jsx";

import "./Index.scss";

function Media(props) {
  const { id } = useParams();

  const { auth, fetchExtraMediaInfo } = props;
  const { token } = auth;

  useEffect(() => {
    console.log("[Media / Index] fetchExtraMediaInfo");
    fetchExtraMediaInfo(token, id);
  }, [fetchExtraMediaInfo, id, token]);

  useEffect(() => {
    const { fetched, error, info } = props.media_info;

    // FETCH_MEDIA_INFO_OK
    if (fetched && !error) {
      document.title = `Dim - ${info.name}`;
    }
  }, [props.media_info]);

  return (
    <div className="mediaPage">
      <Banner/>
      <MetaContent/>
      {props.extra_media_info.info.seasons && (
        <Seasons/>
      )}
    </div>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
  media_info: state.card.media_info,
  extra_media_info: state.card.extra_media_info,
});

const mapActionsToProps = {
  fetchExtraMediaInfo,
  fetchMediaSeasons,
  fetchMediaSeasonEpisodes
};

export default connect(mapStateToProps, mapActionsToProps)(Media);
