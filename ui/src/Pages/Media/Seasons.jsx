import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useParams } from "react-router-dom";
import { fetchMediaSeasons } from "../../actions/media.js";

import CardImage from "./CardImage";
import MediaEpisodes from "./Episodes";

import "./Seasons.scss";

function MediaSeasons(props) {
  const { setActiveId } = props;
  const dispatch = useDispatch();

  const {media} = useSelector(store => ({
    media: store.media
  }));

  const { id } = useParams();
  const [season, setSeason] = useState();
  const [prevID, setPrevID] = useState();

  useEffect(() => {
    dispatch(fetchMediaSeasons(id));
  }, [dispatch, id]);

  useEffect(() => {
    if (!media[id].seasons) return;

    const { seasons } = media[id];

    if (prevID !== id) {
      setPrevID(id);
      setSeason(seasons[0].id);
    }
  }, [id, media, prevID]);

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
        {(season !== undefined && prevID === id) && (
          <MediaEpisodes seasonID={season} setActiveId={setActiveId}/>
        )}
      </div>
    );
  }

  return null;
}

export default MediaSeasons;
