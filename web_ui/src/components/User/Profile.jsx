import React, { Component, Fragment } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import LazyImage from "../../Helpers/LazyImage.jsx";

import "./Profile.scss";

class Profile extends Component {
    render() {
        let user = <Fragment/>;

        /*
            * == USER ==
        */

        // FETCH_USER_START
        if (this.props.user.fetching) {
            user = (
                <div className="profile">
                    <div className="profile-icon">
                        <div className="default-icon"></div>
                    </div>
                    <div className="placeholder-text"/>
                </div>
            );
        }

        // FETCH_USER_ERR
        if (this.props.user.fetched && this.props.user.error) {
            user = (
                <div className="profile">
                    <div className="profile-icon">
                        <div className="default-icon"></div>
                    </div>
                    <div className="horizontal-err">
                        <FontAwesomeIcon icon="times-circle"/>
                        <p>FAILED TO FETCH</p>
                    </div>
                </div>
            );
        }

        // FETCH_USER_OK
        if (this.props.user.fetched && !this.props.user.error) {
            const loading = <div className="default-icon"/>;
            const { username, picture, spentWatching } = this.props.user.info;

            user = (
                <div className="profile">
                    <div className="profile-icon">
                        <LazyImage
                            alt=""
                            src={picture}
                            loading={loading}
                        />
                    </div>
                    <div className="info">
                        <h4>{username}</h4>
                        {this.props.hoursSpentWatching &&
                            <h6>{spentWatching}h spent watching</h6>
                        }
                    </div>
                </div>
            );
        }

        return user;
    }
}

const mapStateToProps = (state) => ({
    user: state.user
});

const mapActionsToProps = {};

export default connect(mapStateToProps, mapActionsToProps)(Profile);
