import { useSelector } from "react-redux";
import { useParams } from "react-router";

import BannerImage from "./BannerImage";

import "./Banner.scss";

function Banner() {
  const media = useSelector(store => store.media);

  const { id } = useParams();

  return (
    <BannerImage src={media[id]?.info.data.backdrop_path}/>
  );
}

export default Banner;
