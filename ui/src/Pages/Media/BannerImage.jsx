import DimLogo from "../../assets/DimLogo";
import ImageLoad from "../../Components/ImageLoad";

const BannerImage = (props) => (
  <div className="bannerImageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideImage">
      {(imageSrc, loaded, error) => {
        if (loaded && !error) return (
          <img src={imageSrc} alt="banner"/>
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
