import { Link } from "react-router-dom";

import { useAppSelector } from "../../../hooks/store";
import { DashboardPoster } from "../../../api/v1/dashboard";
import ProgressBar from "./ProgressBar";
import Image from "./Image";
import TruncText from "../../../Helpers/TruncText";
import NewLibraryModal from "../../../Modals/NewLibrary/Index";
import SelectMediaFile from "../../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../../Modals/SelectMediaFile/Activators/PlayButton";
import CircleIcon from "../../../assets/Icons/Circle";

import "./Banner.scss";

interface Props {
  data?: DashboardPoster;
  isError: boolean;
  isFetching: boolean;
}

function Banner({ data, isError, isFetching }: Props) {
  const { libraries, user } = useAppSelector((store) => ({
    libraries: store.library.fetch_libraries,
    user: store.user,
  }));

  if (isFetching || isError) {
    return (
      <div className="banner">
        <div className="placeholder" />
      </div>
    );
  } else {
    if (!data && libraries.fetched && libraries.items.length > 0) {
      return (
        <div className="banner">
          <div className="placeholder">
            <h2>Your libraries are empty</h2>
            <p>
              Populate the folders they are pointing to with media or add
              another library with existing media
            </p>
            {user.info.roles?.includes("owner") && (
              <NewLibraryModal>
                <button>Add another library</button>
              </NewLibraryModal>
            )}
          </div>
        </div>
      );
    }

    if (!data) {
      return (
        <div className="banner">
          <div className="placeholder">
            <h2>Add your first library</h2>
            <p>
              You will be able to see all the media from your libraries here,
              organized for quick and easy access.
            </p>
            {user.info.roles?.includes("owner") && (
              <NewLibraryModal>
                <button>Add library</button>
              </NewLibraryModal>
            )}
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
      episode,
    } = data;

    const progressBarData = {
      season,
      episode,
      duration,
      delta,
    };

    return (
      <div className="banner">
        <Image src={backdrop} />
        <div className="extras">
          <Link to={`/search?year=${year}`}>{year}</Link>
          {genres.length > 0 && <CircleIcon />}
          {genres.map((genre, i) => (
            <Link to={`/search?genre=${encodeURIComponent(genre)}`} key={i}>
              {genre}
            </Link>
          ))}
        </div>
        <div className="info">
          <h1>{title}</h1>
          <p className="description">
            <TruncText content={synopsis} max={35} />
          </p>
          <SelectMediaFile title={title} mediaID={id}>
            <SelectMediaFilePlayButton
              progress={delta}
              seasonep={{ season, episode }}
            />
          </SelectMediaFile>
        </div>
        <ProgressBar data={progressBarData} />
      </div>
    );
  }
}

export default Banner;
