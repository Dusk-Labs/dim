import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import Banner from "./Banner.jsx";
import { fetchBanners } from "../actions/banner.js";

import "./BannerPage.scss";

class BannerPage extends Component {
    constructor(props) {
        super(props);

        this.handleWS = this.handleWS.bind(this);

        this.state = {
            activeIndex: 0,
            interval: 14000
        };

        this.interval = setInterval(this.next.bind(this), this.state.interval);
    }

    componentDidMount() {
        this.props.fetchBanners(this.props.auth.token);

        if (window.location.protocol !== "https:") {
            this.library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
            this.library_ws.addEventListener("message", this.handleWS);
        }
    }

    componentWillUnmount() {
        clearInterval(this.interval);

        this.library_ws.removeEventListener("message", this.handle_ws_msg);
        this.library_ws.close();
    }

    // TODO: doesn't work with large folders etc -> re-do later, fetchin' before the media is ready.
    handleWS(event) {
        const { type }= JSON.parse(event.data);

        if (type === "EventRemoveLibrary") {
            this.props.fetchBanners(this.props.auth.token);
        }

        if (type === "EventNewLibrary") {
            this.props.fetchBanners(this.props.auth.token);
        }
    }

    next = async () => {
        // FETCH_BANNERS_OK
        if (this.props.banners.fetched && !this.props.banners.error) {
            if (this.props.banners.items.length > 0) {
                let { length } = this.props.banners.items;
                const index = this.state.activeIndex;
                const nextIndex = index < --length ? index + 1 : 0

                this.setState({
                    activeIndex: nextIndex
                });
            } else clearInterval(this.interval);
        }
    }

    toggle = async (e) => {
        // FETCH_BANNERS_OK
        if (this.props.banners.fetched && !this.props.banners.error) {
            clearInterval(this.interval);

            this.setState({
                activeIndex: parseInt(e.currentTarget.dataset.key)
            });

            this.interval = setInterval(this.next.bind(this), this.state.interval);
        }
    }

    render() {
        const crumbs = [];
        let banners = <div className="placeholder"/>;

        // FETCH_BANNERS_ERR
        if (this.props.banners.fetched && this.props.banners.error) {
            banners = (
                <div className="placeholder">
                    <div className="vertical-err">
                        <FontAwesomeIcon icon="times-circle"/>
                        <p>FAILED TO LOAD BANNER</p>
                    </div>
                </div>
            );
        }

        // FETCH_BANNERS_OK
        if (this.props.banners.fetched && !this.props.banners.error) {
            if (this.props.banners.items.length > 0) {
                const { activeIndex } = this.state;

                banners = this.props.banners.items.map((banner, i) => (
                    <div className={activeIndex === i ? "active" : "hide"} key={i}>
                        <Banner key={i} banner={banner}/>
                    </div>
                ));

                // eslint-disable-next-line
                for (let x = 0; x < banners.length; x++) {
                    crumbs.push(
                        <span
                            className={activeIndex === x ? "active" : "hidden"}
                            key={x}
                            data-key={x}
                            onClick={this.toggle}
                        ></span>
                    );
                }
            } else {
                banners = (
                    <div className="placeholder">
                        <div className="vertical-err">
                            <p>Empty</p>
                        </div>
                    </div>
                );
            }
        }

        return (
            <div className="banner-wrapper">
                <div className="pages">{banners}</div>
                <div className="crumbs">{crumbs}</div>
            </div>
        );
    }
}

const mapStateToProps = (state) => ({
    auth: state.auth,
    banners: state.banner
});

const mapActionstoProps = { fetchBanners };

export default connect(mapStateToProps, mapActionstoProps)(BannerPage);
