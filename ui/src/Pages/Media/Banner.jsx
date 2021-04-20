import React from "react";
import { useSelector } from "react-redux";

import BannerImage from "./BannerImage";
import CardImage from "./CardImage";

import "./Banner.scss";

function Banner() {
  const media_info = useSelector(store => store.card.media_info);

  const { poster_path, backdrop_path } = media_info.info;

  return (
    <div className="backdrop">
      <CardImage src={poster_path}/>
      <BannerImage src={backdrop_path}/>
    </div>
  );
}

export default Banner;
