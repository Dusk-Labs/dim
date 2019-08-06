import React, { Component } from 'react';
import "./sidebar.css";

class Sidebar extends Component {
    render() {
        return (
            <div className="sidebar">
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

                    <section classname="search">
                        <form>
                            <div className="search-box">
                                <input type="text" name="search" placeholder="SEARCH"/>
                                <button type="submit">
                                    <i className="fas fa-search"></i>
                                </button>
                            </div>
                        </form>
                    </section>

                </div>

                <div className="center">
                    <section className="connected-hosts">
                        <h4>CONNECTED HOSTS </h4>
                        <div className="list">
                            <i className="fas fa-desktop"></i><a href="#">Desktop</a>
                            <i className="fas fa-laptop"></i><a href="#">Laptop</a>
                            <i className="fas fa-mobile-alt"></i><a href="#">Phone</a>
                        </div>
                    </section>

                    <section className="local-libraries">
                        <h4>LOCAL LIBRARIES </h4>
                        <div className="list">
                            <i className="fas fa-film"></i><a href="#">Movies</a>
                            <i className="fas fa-gamepad"></i><a href="#">Games</a>
                            <i className="fas fa-tv"></i><a href="#">TV Shows</a>
                        </div>
                    </section>

                    <section className="account">
                        <h4>YOUR ACCOUNT</h4>
                        <h4><a href="#">Preferences</a></h4>
                        <h4><a href="#">Legal</a></h4>
                        <h4><a href="#">Logout</a></h4>
                    </section>
                </div>
            </div>
        );
    }
}

export default Sidebar;
