import React, { Component, Fragment } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchInvites, createNewInvite } from "../actions/auth.js";

import Profile from "../Components/User/Profile.jsx";

import "./Preferences.scss";

class Preferences extends Component {
    constructor(props) {
        super(props);

        this.switchTo = this.switchTo.bind(this);

        this.state = {
            active: 0
        }
    }

    componentDidMount() {
        document.title = "Dim - Preferences";

        if (this.props.user.info.owner) {
            this.props.fetchInvites(this.props.auth.token);
        }
    }

    componentDidUpdate() {}

    switchTo(index) {
        this.setState({
            active: index
        });
    }

    // TODO: improve nav switch - just added somethin' super basic to make it work.
    // TODO: track newly created issues better .
    render() {
        let invitesList = <p>You do not have permission to manage invites.</p>;

        if (!this.props.user.owner) {
            const { fetchInvites, createNewInvite } = this.props.auth;

            const invites = [
                ...fetchInvites.items,
                createNewInvite.code
            ];

            // FETCH_INVITES_START
            if (fetchInvites.fetching) {
                invitesList = (
                    <div className="invitesList">
                        <div className="item-wrapper">
                            <div className="placeholder"/>
                        </div>
                        <div className="item-wrapper">
                            <div className="placeholder"/>
                        </div>
                        <div className="item-wrapper">
                            <div className="placeholder"/>
                        </div>
                    </div>
                );
            }

            // FETCH_INVITES_ERR
            if (!fetchInvites.fetching && fetchInvites.error) {
                invitesList = (
                    <div className="item-wrapper">
                        <div className="horizontal-err">
                            <FontAwesomeIcon icon="times-circle"/>
                            <p>FAILED TO FETCH</p>
                        </div>
                    </div>
                );
            }

            // FETCH_INVITES_OK
            if (!fetchInvites.fetching && fetchInvites.fetched && !fetchInvites.error) {
                const invitesElement = invites.map((invite, i) => <p key={i}>{invite}</p>);

                invitesList = (
                    invites.length === 0
                        ? <p>You don't have any invite codes.</p>
                        : <div className="invitesList">{invitesElement}</div>
                );
            }

            // CREATE_NEW_INVITE_ERR
            if (!createNewInvite.creating && createNewInvite.error) {
                console.log(createNewInvite.error);
            }

            // CREATE_NEW_INVITE_OK
            if (!createNewInvite.creating && createNewInvite.created && !createNewInvite.error) {
                this.props.fetchInvites(this.props.auth.token);
            }
        }

        return (
            <div className="preferencesPage">
                <div className="preferences">
                    <nav>
                        <section>
                            <p>Preferences</p>
                            <div className="fields">
                                <div className={this.state.active === 0 ? "field active" : "field"} onClick={() => this.switchTo(0)}>
                                    <p>Account</p>
                                </div>
                                <div className={this.state.active === 1 ? "field active" : "field"} onClick={() => this.switchTo(1)}>
                                    <p>Invites</p>
                                </div>
                            </div>
                        </section>
                    </nav>
                    <div className="content">
                        {this.state.active === 0 &&
                            <section>
                                <p>My account</p>
                                <div className="account">
                                    <Profile name={true}/>
                                    <FontAwesomeIcon className="edit" icon="edit"/>
                                </div>
                            </section>
                        }
                        {this.state.active === 1 &&
                            <section className="invites">
                                <p>Invite Codes</p>
                                {this.props.user.info.owner &&
                                    <button onClick={() => this.props.createNewInvite(this.props.auth.token)}>Create new invite code</button>
                                }
                                {invitesList}
                            </section>
                        }
                    </div>
                </div>
            </div>
        )
    }
}

const mapStateToProps = (state) => ({
    auth: state.auth,
    user: state.user
});

const mapActionsToProps = {
    fetchInvites,
    createNewInvite
};

export default connect(mapStateToProps, mapActionsToProps)(Preferences);
