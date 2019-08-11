import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./sidebar.scss";

import { library } from "@fortawesome/fontawesome-svg-core";

import {
    faDesktop,
    faLaptop,
    faMobileAlt,
    faFilm,
    faGamepad,
    faTv,
    faWrench,
    faDoorOpen,
    faSearch
} from "@fortawesome/free-solid-svg-icons";

library.add(
    faDesktop,
    faLaptop,
    faMobileAlt,
    faFilm,
    faGamepad,
    faTv,
    faWrench,
    faDoorOpen,
    faSearch
);

class Sidebar extends Component {
    render() {
        return (
            <aside className="sidebar">
                <div className="top">
                    <section className="profile">
                        <div className="icon-outer">
                            <div className="icon-inner"></div>
                        </div>
                        <div className="info">
                            <h4 className="profile-name">Username</h4>
                            <h6 className="profile-hours">0h spent watching</h6>
                        </div>
                    </section>

                    <div className="separator"></div>

                    <section className="search">
                        <form>
                            <div className="search-box">
                                <input type="text" name="search" placeholder="SEARCH"/>
                                <button type="submit">
                                    <FontAwesomeIcon icon="search"/>
                                </button>
                            </div>
                        </form>
                    </section>
                </div>

                <div className="middle">
                    <section className="connected-hosts">
                        <div className="header">
                            <h4>CONNECTED HOSTS</h4>
                        </div>
                        <div className="list">
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="desktop"/>
                                    <p>Desktop</p>
                                </a>
                                <a href="http://example.com/">-</a>
                            </div>
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="laptop"/>
                                    <p>Laptop</p>
                                </a>
                                <a href="http://example.com/">-</a>
                            </div>
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="mobile-alt"/>
                                    <p>Phone</p>
                                </a>
                                <a href="http://example.com/">-</a>
                            </div>
                        </div>
                    </section>

                    <section className="local-libraries">
                        <div className="header">
                            <h4>LOCAL LIBRARIES</h4>
                            <a href="http://example.com/">+</a>
                        </div>
                        <div className="list">
                            <div className="item-wrapper">
                                <a className="active" href="http://example.com/">
                                    <FontAwesomeIcon icon="film"/>
                                    <p>Movies</p>
                                </a>
                                <a href="http://example.com/">-</a>
                            </div>
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="gamepad"/>
                                    <p>Games</p>
                                </a>
                                <a href="http://example.com/">-</a>
                            </div>
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="tv"/>
                                    <p>TV Shows</p>
                                </a>
                                <a href="http://example.com/">-</a>
                            </div>
                        </div>
                    </section>

                    <section className="account">
                        <div className="header">
                            <h4>YOUR ACCOUNT</h4>
                        </div>
                        <div className="list">
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="wrench"/>
                                    <p>Preferences</p>
                                </a>
                            </div>
                            <div className="item-wrapper">
                                <a href="http://example.com/">
                                    <FontAwesomeIcon icon="door-open"/>
                                    <p>Logout</p>
                                </a>
                            </div>
                        </div>
                    </section>
                </div>
            </aside>
        );
    }
}

export default Sidebar;
