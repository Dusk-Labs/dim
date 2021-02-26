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

  const [ season, setSeason ] = useState();

  useEffect(() => {
    episodes.current?.scrollIntoView({behavior: "smooth"});
  }, [season]);

  const showSeason = useCallback(number => {
    setSeason(number);
  }, []);

  let mediaSeasons;
  let mediaEpisodes = {};

  if (props.extra_media_info.fetched && !props.extra_media_info.error) {
    if (props.extra_media_info.info.seasons) {
      const { seasons } = props.extra_media_info.info;

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

        mediaEpisodes[seasons[x].season_number] = seasons[x].episodes.map((episode, i) => {
          return (
            <Link to={`/play/${episode.id}`} className="episode" key={i}>
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
