import DimLogo from "../../assets/DimLogo";
import ImageLoad from "../../Components/ImageLoad";

const BannerImage = (props) => (
  <div className="mediaBannerImageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideImage">
      {({imageSrc, loaded, error, setErr}) => {
        if (loaded && !error) return (
          <img src={imageSrc} alt="banner" onError={() => setErr(true)}/>
        );
        if (error) return (
          <div className="placeholder">
            <DimLogo/>
          </div>
        );
      }}
    </ImageLoad>
  </div>
);

export default BannerImage;
