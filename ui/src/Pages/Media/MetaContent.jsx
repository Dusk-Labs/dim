import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useHistory } from "react-router";
import { Link, useParams } from "react-router-dom";

import Button from "../../Components/Misc/Button";
import { fetchMediaInfo } from "../../actions/media";
import CircleIcon from "../../assets/Icons/Circle";
import SelectMediaFile from "../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../Modals/SelectMediaFile/Activators/PlayButton";
import CardImage from "./CardImage";

import "./MetaContent.scss";

function MetaContent() {
  const dispatch = useDispatch();
  const history = useHistory();

  const media = useSelector(store => (
    store.media
  ));

  const { id } = useParams();

  useEffect(() => {
    dispatch(fetchMediaInfo(id));
  }, [dispatch, id]);

  useEffect(() => {
    if (!media[id]?.info) return;

    const { fetched, error, data } = media[id].info;

    // FETCH_MEDIA_INFO_OK
    if (fetched && !error) {
      document.title = `Dim - ${data.name}`;
    }
  }, [id, media]);

  let metaContent = <></>;

  // FETCH_MEDIA_INFO_OK
  if (media[id]?.info?.fetched && media[id]?.info?.error) {
    metaContent = (
      <div className="metaContentErr">
        <h2>Failed to load media</h2>
        <p className="desc">Something went wrong somewhere.</p>
        <Button onClick={history.goBack}>Go back</Button>
      </div>
    );
  }

  // FETCH_MEDIA_INFO_OK
  if (media[id]?.info?.fetched && !media[id]?.info?.error) {
    const {
      description,
      genres,
      name,
      duration,
      rating,
      year,
      media_type,
      seasons,
      progress,
      season,
      episode,
      audio,
      video
    } = media[id].info.data;

    const length = {
      hh: ("0" + Math.floor(duration / 3600)).slice(-2),
      mm: ("0" + Math.floor((duration % 3600) / 60)).slice(-2),
      ss: ("0" + Math.floor((duration % 3600) % 60)).slice(-2)
    };

    metaContent = (
      <div className="metaContent">
        <CardImage src={media[id]?.info.data.poster_path}/>
        <h1>{name}</h1>
        <div className="genres">
          <Link to={`/search?year=${year}`}>{year}</Link>
          {genres.length > 0 && (
            <CircleIcon/>
          )}
          {genres &&
            genres.map((genre, i) => <Link to={`/search?genre=${encodeURIComponent(genre)}`} key={i}>{genre}</Link>)
          }
        </div>
        <p className="description">{description}</p>
        <div className="meta-info">
          <div className="info">
            <h4>Video</h4>
            <p>{video}</p>
          </div>
          <div className="info">
            <h4>Audio</h4>
            <p>{audio}</p>
          </div>
          {!seasons && (
            <div className="info">
              <h4>Duration</h4>
              <p>{length.hh}:{length.mm}:{length.ss}</p>
            </div>
          )}
          {seasons && (
            <div className="info">
              <h4>Seasons</h4>
              <p>{seasons}</p>
            </div>
          )}
          <div className="info">
            <h4>Rating</h4>
            <p>{rating}/10</p>
          </div>
        </div>
        {media_type !== "tv" && (
          <SelectMediaFile title={name} mediaID={id}>
            <SelectMediaFilePlayButton progress={progress} seasonep={{season, episode}}/>
          </SelectMediaFile>
        )}
      </div>
    );
  }

  return metaContent;
}

export default MetaContent;
