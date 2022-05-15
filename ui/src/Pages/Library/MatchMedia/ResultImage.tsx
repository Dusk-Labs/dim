import DimLogo from "assets/DimLogo";
import ImageLoad from "Components/ImageLoad";

interface Props {
  src?: string;
}

const Image = (props: Props) => {
  return (
    <div className="cardImageWrapper">
      <ImageLoad src={props.src}>
        {({ imageSrc, loaded, error, setErr }) => (
          <>
            {loaded && !error && (
              <img src={imageSrc!} alt="cover" onError={() => setErr(true)} />
            )}
            {(error || !props.src || !loaded) && (
              <div className="placeholder">
                <DimLogo />
              </div>
            )}
          </>
        )}
      </ImageLoad>
    </div>
  );
};

export default Image;
