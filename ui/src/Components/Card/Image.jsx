
import DimLogo from "../../assets/DimLogo";
import ImageLoad from "../ImageLoad";

const CardImage = (props) => (
  <div className="cardImageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideImage">
      {({imageSrc, loaded, error, setErr}) => (
        <>
          {loaded && !error && (
            <img src={imageSrc} alt="cover" loading="lazy" onError={() => setErr(true)}/>
          )}
          {error && (
            <div className="placeholder">
              <DimLogo/>
            </div>
          )}
          {props.progress !== undefined && (
            <div className="progress">
              <div className="value" style={{width: `${props.progress | 0}%`}}/>
            </div>
          )}
        </>
      )}
    </ImageLoad>
  </div>
);

export default CardImage;
