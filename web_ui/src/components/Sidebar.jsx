import React, { Component, Fragment } from "react";
import { NavLink } from "react-router-dom";
import Modal from "react-modal";
import { connect } from "react-redux";
import { Scrollbar } from "react-scrollbars-custom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchLibraries, delLibrary, handleWsNewLibrary, handleWsDelLibrary } from "../actions/libraryActions.js";
import { fetchUser } from "../actions/userActions.js";
import { logout } from "../actions/authActions.js";

import SidebarSearch from "../Helpers/SidebarSearch.jsx";
import SidebarIcon from "../Helpers/SidebarIcon.jsx";
import LazyImage from "../Helpers/LazyImage.jsx";
import NewLibraryModal from "../Helpers/NewLibraryModal.jsx";
import ConfirmationBox from "../Helpers/ConfirmationBox.jsx";

import "./Sidebar.scss";

Modal.setAppElement("body");

class Sidebar extends Component {
    constructor(props) {
        super(props);

        this.sidebar = React.createRef();

        this.toggleSidebar = this.toggleSidebar.bind(this);

        if (window.location.protocol !== "https:") {
            this.library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
            this.library_ws.addEventListener("message", this.handle_ws_msg);
        }

        this.state = {show: true};
    }

    handle_ws_msg = async ({data}) => {
        const payload = JSON.parse(data);

        switch(payload.type) {
            case "EventRemoveLibrary":
                this.props.handleWsDelLibrary(payload.id);
                break;
            case "EventNewLibrary":
                this.props.handleWsNewLibrary(this.props.auth.token, payload.id);
                break;
            default:
                break;
        }
    };

    componentDidMount() {
        this.props.fetchUser(this.props.auth.token);
        this.props.fetchLibraries(this.props.auth.token);
    }

    componentWillUnmount() {
        this.library_ws.removeEventListener("message", this.handle_ws_msg);
        this.library_ws.close();
    }

    toggleSidebar() {
        this.setState({
            show: !this.state.show
        });

        const main = document.querySelectorAll("main")[0];

        this.sidebar.current.classList.toggle("hide", this.state.show);
        this.sidebar.current.classList.toggle("show", !this.state.show);

        main.classList.toggle("full", this.state.show);
        main.classList.toggle("shrunk", !this.state.show);
    }

    render() {
        let user;
        let libraries;

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
                        <h6>{spentWatching}h spent watching</h6>
                    </div>
                </div>
            );
        }

        /*
            * == LIBRARIES ==
        */

        // FETCH_LIBRARIES_START
        if (this.props.libraries.fetching) {
            libraries = (
                <Fragment>
                    <div className="item-wrapper">
                        <div className="placeholder"/>
                    </div>
                    <div className="item-wrapper">
                        <div className="placeholder"/>
                    </div>
                </Fragment>
            );
        }

        // FETCH_LIBRARIES_ERR
        if (this.props.libraries.fetched && this.props.libraries.error) {
            libraries = (
                <div className="item-wrapper">
                    <div className="horizontal-err">
                        <FontAwesomeIcon icon="times-circle"/>
                        <p>FAILED TO FETCH</p>
                    </div>
                </div>
            );
        }

        // FETCH_LIBRARIES_OK
        if (this.props.libraries.fetched && !this.props.libraries.error) {
            const { items } = this.props.libraries;

            if (items.length > 0) {
                libraries = items.map((
                    { name, id, media_type }, i
                ) => {
                    const data = {
                        action: "delete",
                        message: `Delete library '${name}'.`,
                        continue: () => {
                            this.props.delLibrary(this.props.auth.token, id);
                        }
                    };

                    return (
                        <div className="item-wrapper" key={i}>
                            <NavLink to={"/library/" + id}>
                                <SidebarIcon icon={media_type || name}/>
                                <p className="item-wrapper-name">{name}</p>
                            </NavLink>
                            <ConfirmationBox {...data}/>
                        </div>
                    )
                });
            } else {
                libraries = (
                    <div className="item-wrapper">
                        <div className="horizontal-err">
                            <p>NO LIBRARIES</p>
                        </div>
                    </div>
                );
            }
        }

        return (
            <nav className="sidebar" ref={this.sidebar}>
                 <div className="toggle" onClick={this.toggleSidebar}>
                    <FontAwesomeIcon icon="angle-left"/>
                </div>
                <section className="main-part">
                    {user}
                    <div className="separator"/>
                    <SidebarSearch/>
                </section>
                <section className="libraries">
                    <header>
                        <h4>LIBRARIES</h4>
                        <NewLibraryModal/>
                    </header>
                    <div className="list">
                        <Scrollbar>
                            <div className="item-wrapper">
                                <NavLink to="/" exact>
                                    <SidebarIcon icon="dashboard"/>
                                    <p className="item-wrapper-name">Dashboard</p>
                                </NavLink>
                            </div>
                            {libraries}
                        </Scrollbar>
                    </div>
                </section>
                <section className="your-account">
                    <header>
                        <h4>YOUR ACCOUNT</h4>
                    </header>
                    <div className="list">
                        <div className="item-wrapper">
                            <NavLink to="/preferences">
                                <SidebarIcon icon="preferences"/>
                                <p className="item-wrapper-name">Preferences</p>
                            </NavLink>
                        </div>
                        <div className="item-wrapper">
                            <NavLink to="/login" onClick={this.props.logout}>
                                <SidebarIcon icon="logout"/>
                                <p className="item-wrapper-name">Logout</p>
                            </NavLink>
                        </div>
                    </div>
                </section>
            </nav>
        );
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
    user: state.userReducer,
    libraries: state.libraryReducer.fetch_libraries
});

const mapActionsToProps = {
    logout,
    fetchLibraries,
    fetchUser,
    delLibrary,
    handleWsDelLibrary,
    handleWsNewLibrary,
};

export default connect(mapStateToProps, mapActionsToProps)(Sidebar);
