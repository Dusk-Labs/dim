import type { Dispatch, SetStateAction } from "react";

import ImageLoad from "Components/ImageLoad";

interface Props {
  src: string;
}

const BannerImage = (props: Props) => (
  <div className="imageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideBannerImage">
      {({
        imageSrc,
        error,
        setErr,
      }: {
        imageSrc: string | null;
        error: boolean;
        setErr: Dispatch<SetStateAction<boolean>>;
      }) => {
        if (error) {
          return <div className="placeholder" />;
        } else if (imageSrc != null) {
          return (
            <img
              src={imageSrc}
              key={imageSrc}
              aria-label="banner"
              onError={() => setErr(true)}
            />
          );
        }
      }}
    </ImageLoad>
  </div>
);

export default BannerImage;
