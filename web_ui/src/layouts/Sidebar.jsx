import React, { Component } from "react";
import { Link } from "react-router-dom";
import Modal from "react-modal";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./sidebar.scss";

Modal.setAppElement("body");

class Sidebar extends Component {
    constructor(props) {
        super(props);

        this.openShowAddLibrary = this.openShowAddLibrary.bind(this);
        this.closeShowAddLibrary = this.closeShowAddLibrary.bind(this);

        this.state = {
            profile: {
                username: null,
                picture: null,
                spentWatching: null
            },
            showAddLibrary: false,
            lists: {
                connected_hosts: [],
                libraries: []
            }
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

    componentDidMount() {
        const profile = {
            username: "Lindsey Morgan",
            picture: "https://frostsnow.com/uploads/biography/2016/01/23/lindsey-morgan.jpg",
            spentWatching: 4
        };

        const hosts = [
            { name: "Desktop", icon: "desktop", path: "/"},
            { name: "Laptop", icon: "laptop", path: "/"},
            { name: "Phone", icon: "mobile-alt", path: "/"}
        ];

        const libs = [
            { name: "Movies", icon: "film", path: "/library/movies"},
            { name: "Games", icon: "gamepad", path: "/library/games"},
            { name: "TV Shows", icon: "tv", path: "/library/tv-shows"}
        ];

        this.setState({
            profile,
            lists: {
                connected_hosts: this.list(hosts),
                libraries: this.list(libs)
            }
        });

    }

    list(items) {
        return items.length !== 0 ? (
            <div className="list">
                {items.map(({ name, icon, path }, i) => {
                    return (
                        <div className="item-wrapper" key={i}>
                            <Link to={path}>
                                <FontAwesomeIcon icon={icon}/>
                                <p>{name}</p>
                            </Link>
                            <button>-</button>
                        </div>
                    );
                })}
            </div>
        ) : (
            <div className="empty">
                <p>CURRENTLY EMPTY</p>
            </div>
        );
    }

    render() {
        const { profile } = this.state;

        return (
            <nav className="sidebar">
                <section className="main-part">
                    <div className="profile">
                        <img alt="icon" src={profile.picture}></img>
                        <div className="info">
                            <h4>{profile.username || "Username"}</h4>
                            <h6>{profile.spentWatching || "0"}h spent watching</h6>
                        </div>
                    </div>
                    <div className="separator"></div>
                    <form>
                        <div className="search-box">
                            <input type="text" name="search" placeholder="SEARCH"/>
                            <button type="submit">
                                <FontAwesomeIcon icon="search"/>
                            </button>
                        </div>
                    </form>
                </section>

                <section className="connected-hosts">
                    <header>
                        <h4>CONNECTED HOSTS</h4>
                    </header>
                    {this.state.lists.connected_hosts}
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
                    {this.state.lists.libraries}
                </section>

                <section className="your-account">
                    <header>
                        <h4>YOUR ACCOUNT</h4>
                    </header>
                    <div className="list">
                        <div className="item-wrapper">
                            <Link to="">
                                <FontAwesomeIcon icon="wrench"/>
                                <p>Preferences</p>
                            </Link>
                        </div>
                        <div className="item-wrapper">
                            <Link to="">
                                <FontAwesomeIcon icon="door-open"/>
                                <p>Logout</p>
                            </Link>
                        </div>
                    </div>
                </section>
            </nav>
        );
    }
}

export default Sidebar;
