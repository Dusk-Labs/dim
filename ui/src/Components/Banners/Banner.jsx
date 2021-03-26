import React from "react";
import { Link } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { connect } from "react-redux";

import ProgressBar from "./ProgressBar.jsx";
import Image from "./Image.jsx";
import PlayButton from "./PlayButton.jsx";
import TruncText from "../../Helpers/TruncText.jsx";
import NewLibraryModal from "../../Modals/NewLibrary/Index";

import "./Banner.scss";

function Banner(props) {
  // FETCH_BANNERS_FETCHING or FETCH_BANNERS_ERROR
  if (props.banners.fetching || (props.banners.fetched && props.banners.error)) {
    return (
      <div className="banner">
        <div className="placeholder"/>
      </div>
    );
  }

  // FETCH_BANNERS_FETCHED
  if (props.banners.fetched && !props.banners.error) {
    if (!props.data) {
      return (
        <div className="banner">
          <div className="placeholder">
            <h2>Add your first library</h2>
            <p>
              You will be able to see all the media from your
              libraries here, organized for quick and easy access.
            </p>
            <NewLibraryModal>
              <button>Add library</button>
            </NewLibraryModal>
          </div>
        </div>
      );
    }

    const {
      title,
      year,
      synopsis,
      backdrop,
      delta,
      duration,
      genres,
      season,
      episode
    } = props.data;

    const progressBarData = {
      season,
      episode,
      duration,
      delta
    };

    return (
      <div className="banner">
        <Image
          src={backdrop}
          hideAnimationName="onHideBannerImage"
        />
        <div className="extras">
          <Link to={`/search?year=${year}`}>{year}</Link>
          {genres.length > 0 && (
            <FontAwesomeIcon icon="circle"/>
          )}
          {genres.map((genre, i) => (
            <Link
              to={`/search?genre=${genre}`}
              key={i}
            >
              {genre}
            </Link>
          ))}
        </div>
        <div className="info">
          <h1>{title}</h1>
          <p className="description">
            <TruncText content={synopsis} max={35}/>
          </p>
          <PlayButton versions={props.data.versions}/>
        </div>
        <ProgressBar data={progressBarData}/>
      </div>
    );
  }

  return <div className="banner"/>
}

const mapStateToProps = (state) => ({
  banners: state.banner
});

const mapActionstoProps = {};

export default connect(mapStateToProps, mapActionstoProps)(Banner);

