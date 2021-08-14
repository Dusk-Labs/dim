import ImageLoad from "../ImageLoad";

const BannerImage = (props) => (
  <div className="imageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideBannerImage">
      {(imageSrc, loaded, error) => {
        if (!error) return (
          <img
            src={imageSrc}
            key={imageSrc}
            aria-label="banner"
          />
        );

        if (error) return (
          <div className="placeholder"/>
        );
      }}
    </ImageLoad>
  </div>
);

export default BannerImage;
