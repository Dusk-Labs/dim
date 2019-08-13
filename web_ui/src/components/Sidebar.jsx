import React, { Component } from "react";
import Modal from "react-modal";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./sidebar.scss";

Modal.setAppElement("body");

function List(items) {
    return (
        <div className="list">
            {items.map((
                { name, icon }, i
            ) => {
                return (
                    <div className="item-wrapper" key={i}>
                        <a href="http://example.com/">
                            <FontAwesomeIcon icon={icon}/>
                            <p>{name}</p>
                        </a>
                        <button>-</button>
                    </div>
                );
            })}
        </div>
    )
}

class Sidebar extends Component {
    constructor(props) {
        super(props);

        this.openShowAddLibrary = this.openShowAddLibrary.bind(this);
        this.closeShowAddLibrary = this.closeShowAddLibrary.bind(this);

        this.state = {
            showAddLibrary: false,
            lists: {
                connected_hosts: [],
                libraries: []
            },
            addLibrary: {
                files: []
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
        const hosts = [
            { name: "Desktop", icon: "desktop" },
            { name: "Laptop", icon: "laptop" },
            { name: "Phone", icon: "mobile-alt" }
        ];

        const libs = [
            { name: "Movies", icon: "film" },
            { name: "Games", icon: "gamepad" },
            { name: "TV Shows", icon: "tv" }
        ];

        this.setState({
            lists: {
                connected_hosts: List(hosts),
                libraries: List(libs)
            }
        });

    }

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
                        {this.state.lists.connected_hosts}
                    </section>

                    <section className="local-libraries">
                        <div className="header">
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
                        </div>
                        {this.state.lists.libraries}
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
