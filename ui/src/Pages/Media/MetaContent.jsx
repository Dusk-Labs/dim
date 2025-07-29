import { useEffect } from "react";
import { useHistory } from "react-router";
import { Link, useParams } from "react-router-dom";
import { skipToken } from "@reduxjs/toolkit/query/react";

import { useGetMediaQuery } from "../../api/v1/media";

import Button from "../../Components/Misc/Button";
import CircleIcon from "../../assets/Icons/Circle";
import SelectMediaFile from "../../Modals/SelectMediaFile/Index";
import SelectMediaFilePlayButton from "../../Modals/SelectMediaFile/Activators/PlayButton";
import CardImage from "./CardImage";

import Dropdown from "./Dropdown";

import "./MetaContent.scss";

function MetaContent(props) {
  const { activeId } = props;
  const history = useHistory();

  const { id } = useParams();

  const { data, isError } = useGetMediaQuery(id ? id : skipToken);

  useEffect(() => {
    if (data) {
      document.title = `Dim - ${data.name}`;
    }
  }, [data]);

  let metaContent = <></>;

  if (isError) {
    metaContent = (
      <div className="metaContentErr">
        <h2>Failed to load media</h2>
        <p className="desc">Something went wrong somewhere.</p>
        <Button onClick={history.goBack}>Go back</Button>
      </div>
    );
  }

  if (data) {
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
      tags,
    } = data;

    const length = {
      hh: ("0" + Math.floor(duration / 3600)).slice(-2),
      mm: ("0" + Math.floor((duration % 3600) / 60)).slice(-2),
      ss: ("0" + Math.floor((duration % 3600) % 60)).slice(-2),
    };

    const { video, audio } = tags[activeId] || {};

    metaContent = (
      <div className="metaContent">
        <CardImage src={data.poster_path} />
        <div className="title">
          <h1>{name}</h1>
          <Dropdown />
        </div>
        <div className="genres">
          <Link to={`/search?year=${year}`}>{year}</Link>
          {genres.length > 0 && <CircleIcon />}
          {genres &&
            genres.map((genre, i) => (
              <Link to={`/search?genre=${encodeURIComponent(genre)}`} key={i}>
                {genre}
              </Link>
            ))}
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
              <p>
                {length.hh}:{length.mm}:{length.ss}
              </p>
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
            <SelectMediaFilePlayButton
              progress={progress}
              seasonep={{ season, episode }}
            />
          </SelectMediaFile>
        )}
      </div>
    );
  }

  return metaContent;
}

export default MetaContent;
