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
        if (imageSrc != null) {
          return (
            <img
              src={imageSrc}
              key={imageSrc}
              aria-label="banner"
              onError={() => setErr(true)}
            />
          );
        }

        return <div className="placeholder" />;
      }}
    </ImageLoad>
  </div>
);

export default BannerImage;
