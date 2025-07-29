import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { useGetMediaSeasonsQuery } from "../../api/v1/media";

import CardImage from "./CardImage";
import MediaEpisodes from "./Episodes";

import "./Seasons.scss";

function MediaSeasons(props) {
  const { setActiveId } = props;

  const { id } = useParams();
  const [season, setSeason] = useState();
  const [prevID, setPrevID] = useState();

  const { data: seasons } = useGetMediaSeasonsQuery(id);

  useEffect(() => {
    if (!seasons) return;

    if (prevID !== id) {
      setPrevID(id);
      setSeason(seasons[0].id);
    }
  }, [seasons, id, prevID]);

  if (seasons) {
    return (
      <div className="mediaPageSeasons">
        <section>
          <h2>Seasons</h2>
          <div className={`seasons ${season && "selected"}`}>
            {seasons.map(({ id, season_number, poster }) => (
              <div
                className={`season ${id === season && "active"}`}
                key={id}
                onClick={() => setSeason(id)}
              >
                <CardImage src={poster} />
                <p>Season {season_number}</p>
              </div>
            ))}
          </div>
        </section>
        {season !== undefined && prevID === id && (
          <MediaEpisodes seasonID={season} setActiveId={setActiveId} />
        )}
      </div>
    );
  }

  return null;
}

export default MediaSeasons;
