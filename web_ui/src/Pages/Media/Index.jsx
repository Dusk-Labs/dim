import React, { useCallback, useEffect, useRef, useState } from "react";
import { connect } from "react-redux";
import { Link, useParams } from "react-router-dom";

import {
  fetchExtraMediaInfo,
  fetchMediaSeasons,
  fetchMediaSeasonEpisodes
} from "../../actions/card.js";

import LazyImage from "../../Helpers/LazyImage.jsx";
import Banner from "./Banner";
import MetaContent from "./MetaContent";

import "./Index.scss";

function Media(props) {
  const { id } = useParams();
  const episodes = useRef(null);

  const [ season, setSeason ] = useState();

  useEffect(() => {
    props.fetchExtraMediaInfo(props.auth.token, id);
  }, [id]);

  useEffect(() => {
    const { fetched, error, info } = props.media_info;

    // FETCH_MEDIA_INFO_OK
    if (fetched && !error) {
      document.title = `Dim - ${info.name}`;
    }

    episodes.current?.scrollIntoView();
  }, [props.media_info]);

  const showSeason = useCallback(number => {
    setSeason(number);
  }, []);

  let mediaSeasons;
  let mediaEpisodes = {};

  // FETCH_EXTRA_MEDIA_INFO_ERR
  // if (props.extra_media_info.fetched && props.extra_media_info.error) {
  if (false) {
    console.table("[FETCH EXTRA MEDIA INFO] ERR", props.extra_media_info);
  }

  // FETCH_EXTRA_MEDIA_INFO_OK
  // if (props.extra_media_info.fetched && !props.extra_media_info.error) {
  if (false) {
    if (props.extra_media_info.info.seasons) {
      const { seasons } = props.extra_media_info.info;

      seasons.sort((a, b) => {
        return a.season_number - b.season_number;
      });

      mediaSeasons = seasons.map((season, si) => {
        return (
          <div className="season" key={si} onClick={() => showSeason(season.season_number)}>
            <LazyImage src={season.poster}/>
            <p>SEASON {season.season_number}</p>
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
              <LazyImage src={episode.backdrop}/>
              <p>EPISODE {episode.episode}</p>
            </Link>
          );
        });
      }
    }
  }

  return (
    <div className="media-page">
      <Banner/>
      <MetaContent/>
      {props.extra_media_info.info.seasons && (
        <div className="content">
          <div className="se-ep">
            <div className="seasons">
              <h2>SEASONS</h2>
              <div className="list">
                {mediaSeasons}
              </div>
              {season !== undefined && (
                <div className="episodes" ref={episodes}>
                  <h2>SEASON {season} - EPISODES</h2>
                  <div className="list">
                    {mediaEpisodes[season]}
                  </div>
                </div>
              )}
              </div>
          </div>
        </div>
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
  fetchExtraMediaInfo,
  fetchMediaSeasons,
  fetchMediaSeasonEpisodes
};

export default connect(mapStateToProps, mapActionsToProps)(Media);
