import React from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import LazyImage from "../../../Helpers/LazyImage.jsx";
import ProfileImage from "./Image.jsx";

import "./Index.scss";

function Profile(props) {
  // FETCH_USER_START
  if (props.user.fetching) {
    return (
      <div className="profile">
        <div className="icon loading">
            <div className="placeholder-icon"/>
          </div>
      </div>
    )
  }

  // FETCH_USER_ERR
  if (props.user.fetched && props.user.error) {
    return (
      <div className="profile">
        <div className="icon">
          <div className="error-icon">
            <FontAwesomeIcon icon="times-circle"/>
          </div>
        </div>
        <div className="info">
          <h5>Cannot load data</h5>
        </div>
      </div>
    )
  }

  // FETCH_USER_OK
  if (props.user.fetched && !props.user.error) {
    const { username, picture, spentWatching } = props.user.info;

    return (
      <div className="profile">
        <div className="icon">
          <ProfileImage src={picture}/>
        </div>
        <div className="info">
            <h4>{username || "eray_chumak"}</h4>
            {props.hoursSpentWatching &&
              <h5>Spent {spentWatching || 0}h watching</h5>
            }
        </div>
      </div>
    );
  }

  return <div className="profile"/>;
}

const mapStateToProps = (state) => ({
    user: state.user
});

export default connect(mapStateToProps)(Profile);
