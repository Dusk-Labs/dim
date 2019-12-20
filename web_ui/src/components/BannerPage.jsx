import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import Banner from "../components/Banner.jsx";
import { fetchBanners } from "../actions/bannerActions.js";

import "./BannerPage.scss";

class BannerPage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            activeIndex: 0,
            interval: 14000,
        };

        this.interval = setInterval(this.next.bind(this), this.state.interval);
    }

    componentDidMount() {
        this.props.fetchBanners(this.props.auth.token);
    }

    componentWillUnmount() {
        clearInterval(this.interval);
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
                    <div className="empty">
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
                        <div className="empty">
                            <FontAwesomeIcon icon="times-circle"/>
                            <p>NO BANNERS FOUND</p>
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
    auth: state.authReducer,
    banners: state.bannerReducer
});

const mapActionstoProps = { fetchBanners };

export default connect(mapStateToProps, mapActionstoProps)(BannerPage);
