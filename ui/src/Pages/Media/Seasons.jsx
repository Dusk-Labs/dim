import { useCallback, useEffect, useRef, useState } from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";

import {
  fetchMediaSeasons,
  fetchMediaSeasonEpisodes
} from "../../actions/card.js";

import CardImage from "./CardImage.jsx";

import "./Index.scss";

function Media(props) {
  const episodes = useRef(null);

  const [ scrollSmoothly, setScrollSmoothly ] = useState(false);
  const [ season, setSeason ] = useState();

  useEffect(() => {
    if (!scrollSmoothly) return;
    episodes.current?.scrollIntoView({behavior: "smooth"});
  }, [scrollSmoothly]);

  const { info, fetched, error } = props.extra_media_info;

  useEffect(() => {
    if (info.seasons.length === 1) {
      setSeason(info.seasons[0].season_number);
    }
  }, [info.seasons]);

  const showSeason = useCallback(number => {
    setSeason(number);
    setScrollSmoothly(true);
  }, []);

  let mediaSeasons;
  let mediaEpisodes = {};

  if (fetched && !error) {
    if (info.seasons) {
      const { seasons } = info;

      seasons.sort((a, b) => {
        return a.season_number - b.season_number;
      });

      mediaSeasons = seasons.map(({season_number, poster}, i) => {
        return (
          <div
            className={`season ${season_number === season && "active"}`}
            key={i}
            onClick={() => showSeason(season_number)}
          >
            <CardImage src={poster}/>
            <p>Season {season_number}</p>
          </div>
        );
      });

      for (let x = 0; x < seasons.length; x++) {
        seasons[x].episodes.sort((a, b) => {
          return a.episode - b.episode;
        });

        // TODO: modal selecting which file
        mediaEpisodes[seasons[x].season_number] = seasons[x].episodes.map((episode, i) => {
          return (
            <Link to={`/media/${props.media_info.info.id}/play/${episode.versions[0].id}`} className="episode" key={i}>
              <CardImage src={episode.backdrop}/>
              <p>Episode {episode.episode}</p>
            </Link>
          );
        });
      }
    }
  }

  return (
    <div className="mediaPageSeasons">
      <section>
        <h2>Seasons</h2>
        <div className={`seasons ${season && "selected"}`}>
          {mediaSeasons}
        </div>
      </section>
      {season !== undefined && (
        <section>
          <h2>Episodes</h2>
          <div className="episodes" ref={episodes}>
            {mediaEpisodes[season]}
          </div>
        </section>
      )}
    </div>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
  media_info: state.card.media_info,
  extra_media_info: state.card.extra_media_info,
});

const mapActionsToProps = {
  fetchMediaSeasons,
  fetchMediaSeasonEpisodes
};

export default connect(mapStateToProps, mapActionsToProps)(Media);
