import { Link } from "react-router-dom";
import { useSelector } from "react-redux";

import ProgressBar from "./ProgressBar.jsx";
import Image from "./Image.jsx";
import TruncText from "../../Helpers/TruncText.jsx";
import NewLibraryModal from "../../Modals/NewLibrary/Index";
import SelectMediaFile from "../../Modals/SelectMediaFile/Index.jsx";
import SelectMediaFilePlayButton from "../../Modals/SelectMediaFile/Activators/PlayButton.jsx";
import CircleIcon from "../../assets/Icons/Circle";

import "./Banner.scss";

function Banner(props) {
  const { banners, libraries } = useSelector(store => ({
    banners: store.banner,
    libraries: store.library.fetch_libraries
  }));

  // FETCH_BANNERS_FETCHING or FETCH_BANNERS_ERROR
  if (banners.fetching || (banners.fetched && banners.error)) {
    return (
      <div className="banner">
        <div className="placeholder"/>
      </div>
    );
  }

  // FETCH_BANNERS_FETCHED
  if (banners.fetched && !banners.error) {
    if (!props.data && libraries.fetched && libraries.items.length > 0) {
      return (
        <div className="banner">
          <div className="placeholder">
            <h2>Your libraries are empty</h2>
            <p>
              Populate the folders they are pointing to with
              media or add another library with existing media
            </p>
            <NewLibraryModal>
              <button>Add another library</button>
            </NewLibraryModal>
          </div>
        </div>
      );
    }

    if (!props.data) {
      return (
        <div className="banner">
          <div className="placeholder">
            <h2>Add your first library</h2>
            <p>
              You will be able to see all the media from your
              libraries here, organized for quick and easy access.
            </p>
            <NewLibraryModal>
              <button>Add library</button>
            </NewLibraryModal>
          </div>
        </div>
      );
    }

    const {
      id,
      title,
      year,
      synopsis,
      backdrop,
      delta,
      duration,
      genres,
      season,
      episode
    } = props.data;

    const progressBarData = {
      season,
      episode,
      duration,
      delta
    };

    return (
      <div className="banner">
        <Image src={backdrop}/>
        <div className="extras">
          <Link to={`/search?year=${year}`}>{year}</Link>
          {genres.length > 0 && (
            <CircleIcon/>
          )}
          {genres.map((genre, i) => (
            <Link
              to={`/search?genre=${encodeURIComponent(genre)}`}
              key={i}
            >
              {genre}
            </Link>
          ))}
        </div>
        <div className="info">
          <h1>{title}</h1>
          <p className="description">
            <TruncText content={synopsis} max={35}/>
          </p>
          <SelectMediaFile title={title} mediaID={id}>
            <SelectMediaFilePlayButton progress={delta} seasonep={{season, episode}}/>
          </SelectMediaFile>
        </div>
        <ProgressBar data={progressBarData}/>
      </div>
    );
  }

  return <div className="banner"/>;
}

export default Banner;
