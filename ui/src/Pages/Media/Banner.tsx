import { useParams } from "react-router";

import { useGetMediaQuery } from "api/v1/media";

import BannerImage from "./BannerImage";

import "./Banner.scss";

function Banner() {
  const { id } = useParams<{ id: string }>();

  const { data: media } = useGetMediaQuery(id);

  if (media && media.backdrop_path) {
    return <BannerImage src={media.backdrop_path} />;
  } else {
    return null;
  }
}

export default Banner;
