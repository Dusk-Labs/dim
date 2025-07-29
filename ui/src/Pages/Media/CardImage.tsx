import DimLogo from "assets/DimLogo";
import ImageLoad, { ImageLoadChildrenParams } from "Components/ImageLoad";

interface Props {
  progress?: number;
  src: string;
}

export const CardImage = (props: Props) => (
  <div className="mediaCardImage">
    <ImageLoad src={props.src} triggerAnimation="onHideImage">
      {({ imageSrc, loaded, error, setErr }: ImageLoadChildrenParams) => (
        <>
          {loaded && !error && imageSrc != null && (
            <img src={imageSrc} alt="cover" onError={() => setErr(true)} />
          )}
          {error && (
            <div className="placeholder">
              <DimLogo />
            </div>
          )}
          {props.progress !== undefined && (
            <div className="progress">
              <div
                className="value"
                style={{ width: `${props.progress | 0}%` }}
              />
            </div>
          )}
        </>
      )}
    </ImageLoad>
  </div>
);

export default CardImage;
