import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useParams } from "react-router-dom";
import { fetchMediaSeasons } from "../../actions/media.js";

import CardImage from "./CardImage.jsx";
import MediaEpisodes from "./Episodes.jsx";

import "./Index.scss";

function MediaSeasons() {
  const dispatch = useDispatch();

  const {media} = useSelector(store => ({
    media: store.media
  }));

  const { id } = useParams();
  const [season, setSeason] = useState();

  useEffect(() => {
    dispatch(fetchMediaSeasons(id));
  }, [dispatch, id]);

  useEffect(() => {
    if (!media[id].seasons) return;

    const { seasons } = media[id];

    if (seasons.length > 0) {
      setSeason(seasons[0].id);
    }
  }, [id, media]);

  if (media[id]?.seasons) {
    return (
      <div className="mediaPageSeasons">
        <section>
          <h2>Seasons</h2>
          <div className={`seasons ${season && "selected"}`}>
            {media[id].seasons.map(({id, season_number, poster}, i) => (
              <div
                className={`season ${id === season && "active"}`}
                key={i}
                onClick={() => setSeason(id)}
              >
                <CardImage src={poster}/>
                <p>Season {season_number}</p>
              </div>
            ))}
          </div>
        </section>
        {season !== undefined && (
          <MediaEpisodes seasonID={season}/>
        )}
      </div>
    );
  }

  return null;
}

export default MediaSeasons;
