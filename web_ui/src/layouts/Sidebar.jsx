import React, { Component } from "react";
import { NavLink } from "react-router-dom";
import Modal from "react-modal";
import { connect } from "react-redux";
import { Scrollbar } from "react-scrollbars-custom";

import { fetchLibraries } from "../actions/libraryActions.js";
import { fetchHosts } from "../actions/hostActions.js";
import { fetchUser } from "../actions/userActions.js";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import SidebarSearch from "../helpers/SidebarSearch.jsx";
import SidebarIcon from "../helpers/SidebarIcon.jsx";
import LazyImage from "../helpers/LazyImage.jsx";

import "./Sidebar.scss";

Modal.setAppElement("body");

class Sidebar extends Component {
    constructor(props) {
        super(props);

        this.openShowAddLibrary = this.openShowAddLibrary.bind(this);
        this.closeShowAddLibrary = this.closeShowAddLibrary.bind(this);
        this.library_ws = new WebSocket('ws://86.21.150.167:3012/events/library');
        this.library_ws.addEventListener('message', this.handle_ws_msg);

        this.state = {
            showAddLibrary: false
        };
    }

    openShowAddLibrary() {
        this.setState({
            showAddLibrary: true
        });
    }

    closeShowAddLibrary() {
        this.setState({
            showAddLibrary: false
        });
    }

    async componentDidMount() {
        this.props.fetchUser();
        this.props.fetchHosts();
        this.props.fetchLibraries();
    }

    render() {
        let user;
        let hosts;
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
                    <p id="response">LOADING</p>
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
                    <p id="response">FAILED TO LOAD</p>
                </div>
            );
        }

        // FETCH_USER_OK
        if (this.props.user.fetched && !this.props.user.error) {
            const loading = (
                <div className="default-icon"></div>
            );

            const { username, picture, spentWatching } = this.props.user.info;

            user = (
                <div className="profile">
                    <div className="profile-icon">
                        <LazyImage
                            alt="profile-icon"
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
            * == HOSTS ==
        */

        // FETCH_HOSTS_START
        if (this.props.hosts.fetching) {
            hosts = <p id="response">LOADING</p>;
        }

        // FETCH_HOSTS_ERR
        if (this.props.hosts.fetched && this.props.hosts.error) {
            hosts = <p id="response">FAILED TO LOAD</p>
        }

        // FETCH_HOSTS_OK
        if (this.props.hosts.fetched && !this.props.hosts.error) {
            const { items } = this.props.hosts;

            if (items.length > 0) {
                hosts = items.map((
                    { name, id, media_type }, i
                ) => (
                    <div className="item-wrapper" key={i}>
                        <NavLink to={"/device/" + id}>
                            <SidebarIcon icon={media_type || name}/>
                            <p>{name}</p>
                        </NavLink>
                    </div>
                ));
            } else hosts = <p id="response">NO HOSTS</p>
        }

        /*
            * == LIBRARIES ==
        */

        // FETCH_LIBRARIES_START
        if (this.props.libraries.fetching) {
            libraries = <p id="response">LOADING</p>;
        }

        // FETCH_LIBRARIES_ERR
        if (this.props.libraries.fetched && this.props.libraries.error) {
            libraries = <p id="response">FAILED TO LOAD</p>
        }

        // FETCH_LIBRARIES_OK
        if (this.props.libraries.fetched && !this.props.libraries.error) {
            const { items } = this.props.libraries;

            if (items.length > 0) {
                libraries = items.map((
                    { name, id, media_type }, i
                ) => (
                    <div className="item-wrapper" key={i}>
                        <NavLink to={"/library/" + id}>
                            <SidebarIcon icon={media_type || name}/>
                            <p>{name}</p>
                        </NavLink>
                        <button>-</button>
                    </div>
                ));
            } else libraries = <p id="response">NO LIBRARIES</p>
        }

        return (
            <nav className="sidebar">
                <section className="main-part">
                    { user }
                    <div className="separator"></div>
                    <SidebarSearch></SidebarSearch>
                </section>

                <section className="connected-hosts">
                    <header>
                        <h4>CONNECTED HOSTS</h4>
                    </header>
                    <div className="list">
                        <Scrollbar>
                            { hosts }
                        </Scrollbar>
                    </div>
                </section>

                <section className="local-libraries">
                    <header>
                        <h4>LOCAL LIBRARIES</h4>
                        <button onClick={this.openShowAddLibrary}>+</button>
                        <Modal
                            isOpen={this.state.showAddLibrary}
                            contentLabel="Minimal Modal Example"
                            className="popup"
                            overlayClassName="overlay"
                        >
                            <h2>ADD LIBRARY</h2>
                            <input type="text" name="name" placeholder="NAME"/>
                            <div className="options">
                                <button onClick={this.closeShowAddLibrary}>CANCEL</button>
                                <a href="add-library/post">ADD</a>
                            </div>
                        </Modal>
                    </header>
                    <div className="list">
                        <Scrollbar>
                            <div className="item-wrapper">
                                <NavLink to="/" exact>
                                    <FontAwesomeIcon icon="home"/>
                                    <p>Dashboard</p>
                                </NavLink>
                            </div>
                            { libraries }
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
                                <FontAwesomeIcon icon="wrench"/>
                                <p>Preferences</p>
                            </NavLink>
                        </div>
                        <div className="item-wrapper">
                            <NavLink to="/logout">
                                <FontAwesomeIcon icon="door-open"/>
                                <p>Logout</p>
                            </NavLink>
                        </div>
                    </div>
                </section>
            </nav>
        );
    }
}

const mapStateToProps = (state) => ({
    user: state.user,
    hosts: state.hosts,
    libraries: state.libraries
});

const actions = {
    fetchLibraries,
    fetchHosts,
    fetchUser
};

export default connect(mapStateToProps, actions)(Sidebar);
