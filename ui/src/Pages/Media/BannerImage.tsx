import DimLogo from "assets/DimLogo";
import ImageLoad, { ImageLoadChildrenParams } from "Components/ImageLoad";

interface Props {
  src: string;
}

export const BannerImage = (props: Props) => (
  <div className="mediaBannerImageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideImage">
      {({ imageSrc, loaded, error, setErr }: ImageLoadChildrenParams) => {
        if (loaded && !error && imageSrc != null) {
          return (
            <img src={imageSrc} alt="banner" onError={() => setErr(true)} />
          );
        } else {
          return (
            <div className="placeholder">
              <DimLogo />
            </div>
          );
        }
      }}
    </ImageLoad>
  </div>
);

export default BannerImage;
