import ImageLoad from "../../../Components/ImageLoad";

const BannerImage = (props) => (
  <div className="imageWrapper">
    <ImageLoad src={props.src} triggerAnimation="onHideBannerImage">
      {({ imageSrc, error, setErr }) => {
        if (!error)
          return (
            <img
              src={imageSrc}
              key={imageSrc}
              aria-label="banner"
              onError={() => setErr(true)}
            />
          );

        if (error) return <div className="placeholder" />;
      }}
    </ImageLoad>
  </div>
);

export default BannerImage;
