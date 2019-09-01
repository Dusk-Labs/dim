import React, { Component } from "react";
import { NavLink } from "react-router-dom";
import Modal from "react-modal";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import SidebarIcon from "../helpers/SidebarIcon.jsx";
import "./Sidebar.scss";

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

    async componentDidMount() {
        const profile = {
            username: "Lana Rhoades",
            picture: "https://i.pinimg.com/564x/69/85/38/698538a169f06333ee09a532c91a49d8.jpg",
            spentWatching: 12
        };

        const hosts = [
            { name: "Desktop", id: "1"},
            { name: "Laptop", id: "2"},
            { name: "Phone", id: "3"}
        ];

        // const reqProfile = fetch("http://86.21.150.167:8000/api/v1/");
        // const reqHosts = fetch("http://86.21.150.167:8000/api/v1/");
        const reqLibs = fetch("http://86.21.150.167:8000/api/v1/library/");

        // const [ profile, hosts, libs ] = [
        const [ libs ] = [
            // await (await reqProfile).json(),
            // await (await reqHosts).json(),
            await (await reqLibs).json()
        ];

        this.setState({
            profile,
            lists: {
                connected_hosts: this.listConnectedHosts(hosts),
                libraries: this.listLibraries(libs)
            }
        });

    }

    listConnectedHosts(res) {
        const items = !res || res.error ? [] : res;

        return items.length !== 0 ? (
            <div className="list">
                {items.map(({ name, id, media_type }, i) => {
                    return (
                        <div className="item-wrapper" key={i}>
                            <NavLink to={"/device/" + id}>
                                <SidebarIcon icon={media_type || name}/>
                                <p>{name}</p>
                            </NavLink>
                            <button>-</button>
                        </div>
                    );
                })}
            </div>
        ) : (
            <div className="empty">
                <p>NO HOSTS</p>
            </div>
        );
    }

    listLibraries(res) {
        const items = !res || res.error ? [] : res;

        return items.length !== 0 ? (
            <div className="list">
                <div className="item-wrapper">
                    <NavLink to="/" exact>
                        <FontAwesomeIcon icon="home"/>
                        <p>Dashboard</p>
                    </NavLink>
                </div>
                {items.map(({ name, id, media_type }, i) => {
                    return (
                        <div className="item-wrapper" key={i}>
                            <NavLink to={"/library/" + id}>
                                <SidebarIcon icon={media_type || name}/>
                                <p>{name}</p>
                            </NavLink>
                            <button>-</button>
                        </div>
                    );
                })}
            </div>
        ) : (
            <div className="empty">
                <p>NO LIBRARIES</p>
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
                            <NavLink to="">
                                <FontAwesomeIcon icon="wrench"/>
                                <p>Preferences</p>
                            </NavLink>
                        </div>
                        <div className="item-wrapper">
                            <NavLink to="">
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

export default Sidebar;
