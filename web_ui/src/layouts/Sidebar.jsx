import React, { Component } from "react";
import { NavLink } from "react-router-dom";
import Modal from "react-modal";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import SidebarSearch from "../helpers/SidebarSearch.jsx";
import SidebarIcon from "../helpers/SidebarIcon.jsx";
import LazyImage from "../helpers/LazyImage.jsx";
import ProfileIcon from "../assets/profile_icon.jpg";
import { Scrollbar } from "react-scrollbars-custom";
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
            profile: (
                <div className="profile">
                    <div className="default-icon"></div>
                    <p id="response">LOADING</p>
                </div>
            ),
            showAddLibrary: false,
            connectedHosts: <p id="response">LOADING</p>,
            libraries: []
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

    handle_ws_msg = async (event) => {
        let msg = JSON.parse(event.data);
        if (msg.res !== "/events/library")
            return;
        if (msg.message.event_type.type === "EventNewLibrary") {
            let lib_data = await this.handle_req(fetch(`http://86.21.150.167:8000/api/v1/library/${msg.message.id}`));

            if (lib_data.err === undefined) {
                this.setState({
                    libraries: [...this.state.libraries, lib_data].sort((a, b) => {
                        let name_a = a.name.toUpperCase();
                        let name_b = b.name.toUpperCase();

                        if (name_a < name_b)
                            return -1;

                        if (name_a > name_b)
                            return 1;
                        return 0;
                    })
                });
            }
        } else if (msg.message.event_type.type === "EventRemoveLibrary") {
            let libraries = this.state.libraries.filter(x => x.id !== msg.message.id);
            this.setState({
                libraries
            });
        }
    }

    async handle_req(promise) {
        try {
            return await (await promise).json();
        } catch (err) {
            return { err: err };
        }
    }

    async componentDidMount() {
        this.getProfile();
        this.listConnectedHosts();
        this.listLibraries();
    }

    async getProfile() {
        // ! WHEN IS API READY
        // const reqProfile = fetch("http://86.21.150.167:8000/api/v1/");
        // const profile = await this.handle_req(reqProfile);

        // if (profile.err) {
        //     const profFailed = (
        //         <div className="profile">
        //             <div className="default-icon"></div>
        //             <p id="response">FAILED TO LOAD</p>
        //         </div>
        //     );

        //     return this.setState({
        //         profile: profFailed
        //     });
        // }
        // !

        // ! TEMP (REMOVE WHEN USING API)
        const profile = {
            username: "Lana Rhoades",
            picture: ProfileIcon,
            spentWatching: 12
        };
        // !

        const { username, picture, spentWatching } = profile;

        const loading = (
            <div className="default-icon"></div>
        );

        return this.setState({
            profile: (
                <div className="profile">
                    <div className="profile-icon">
                        <LazyImage alt="profile-icon" src={picture} loading={loading}/>
                    </div>
                    <div className="info">
                        <h4>{username}</h4>
                        <h6>{spentWatching}h spent watching</h6>
                    </div>
                </div>
            )
        });
    }

    async listConnectedHosts() {
        // ! FOR WHEN API READY
        // const reqHosts = fetch("http://86.21.150.167:8000/api/v1/");
        // const hosts = await this.handle_req(reqHosts);

        // if (hosts.err) {
        //     return this.setState({
        //         connectedHosts: <p id="response">FAILED TO LOAD</p>,
        //     });
        // }
        // !

        // ! TEMP (REMOVE WHEN USING API)
        const hosts = [
            { name: "Desktop", id: "1"},
            { name: "Laptop", id: "2"},
            { name: "Phone", id: "3"}
        ];
        // !

        const list = hosts.length !== 0
            ? (
                hosts.map(({ name, id, media_type }, i) => {
                    return (
                        <div className="item-wrapper" key={i}>
                            <NavLink to={"/device/" + id}>
                                <SidebarIcon icon={media_type || name}/>
                                <p>{name}</p>
                            </NavLink>
                        </div>
                    );
                })
            ) : <p id="response">NO HOSTS</p>;

        return this.setState({
            connectedHosts: list
        });
    }

    async listLibraries() {
        const reqLibs = fetch("http://86.21.150.167:8000/api/v1/library");
        const libs = await this.handle_req(reqLibs);

        if (libs.err) {
            return this.setState({
                libraries: []
            });
        }

        return this.setState({
            libraries: libs
        });
    }

    render() {
        const {libraries} = this.state;
        const libs = (
            libraries.length !== 0
                ? libraries.map((
                    { name, id, media_type }, i
                ) =>
                    <div className="item-wrapper" key={i}>
                        <NavLink to={"/library/" + id}>
                            <SidebarIcon icon={media_type || name}/>
                            <p>{name}</p>
                        </NavLink>
                        <button>-</button>
                    </div>
                ) : <p id="response">NO LIBRARIES</p>
        );

        return (
            <nav className="sidebar">
                <section className="main-part">
                    {this.state.profile}
                    <div className="separator"></div>
                    <SidebarSearch></SidebarSearch>
                </section>

                <section className="connected-hosts">
                    <header>
                        <h4>CONNECTED HOSTS</h4>
                    </header>
                    <div className="list">
                        <Scrollbar>
                            {this.state.connectedHosts}
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
                            {libs}
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

export default Sidebar;
