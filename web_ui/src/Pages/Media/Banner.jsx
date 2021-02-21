import React from "react";
import { connect } from "react-redux";

import BannerImage from "./BannerImage";
import CardImage from "./CardImage";

import "./Banner.scss";

function Banner(props) {
  const { poster_path, backdrop_path } = props.media_info.info;

  return (
    <div className="backdrop">
      {poster_path && <CardImage src={poster_path}/>}
      {backdrop_path && <BannerImage src={backdrop_path}/>}
    </div>
  );
}

const mapStateToProps = (state) => ({
  media_info: state.card.media_info,
});

const mapActionsToProps = {};

export default connect(mapStateToProps, mapActionsToProps)(Banner);
