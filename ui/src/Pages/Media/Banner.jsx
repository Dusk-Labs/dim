import { useSelector } from "react-redux";
import { useParams } from "react-router";

import BannerImage from "./BannerImage";
import CardImage from "./CardImage";

import "./Banner.scss";

function Banner() {
  const media = useSelector(store => store.media);

  const { id } = useParams();

  return (
    <div className="backdrop">
      <CardImage src={media[id]?.info.data.poster_path}/>
      <BannerImage src={media[id]?.info.data.backdrop_path}/>
    </div>
  );
}

export default Banner;
