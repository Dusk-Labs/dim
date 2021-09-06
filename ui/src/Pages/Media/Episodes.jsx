import { useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useParams } from "react-router-dom";
import { fetchMediaEpisodes } from "../../actions/media.js";
import SelectMediaFileEpisode from "../../Modals/SelectMediaFile/Activators/Episode.jsx";
import SelectMediaFile from "../../Modals/SelectMediaFile/Index.jsx";

function MediaEpisodes(props) {
  const dispatch = useDispatch();

  const {media} = useSelector(store => ({
    media: store.media
  }));

  const episodesDiv = useRef(null);
  const { id } = useParams();

  useEffect(() => {
    episodesDiv.current?.scrollIntoView({behavior: "smooth"});
  }, []);

  useEffect(() => {
    dispatch(fetchMediaEpisodes(id, props.seasonID));
  }, [dispatch, id, props.seasonID]);

  if (!media[id]?.episodes) return null;

  const { episodes } = media[id];

  return (
    <section>
      <h2>Episodes</h2>
      {episodes.length === 0 && (
        <p className="desc">Empty</p>
      )}
      {episodes.length > 0 && (
        <div className="episodes" ref={episodesDiv}>
          {episodes.map((ep, i) => (
            <SelectMediaFile key={i} title={`Episode ${ep.episode}`} mediaID={ep.id}>
              <SelectMediaFileEpisode number={ep.episode} thumbnail={ep.thumbnail_url}/>
            </SelectMediaFile>
          ))}
        </div>
      )}
    </section>
  );
}

export default MediaEpisodes;
