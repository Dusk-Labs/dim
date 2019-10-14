import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchDashboard } from "../actions/dashboardActions.js";
import { fetchBanners } from "../actions/dashboardActions.js";

import CardList from "../layouts/CardList.jsx";
import Banner from "../components/Banner.jsx";
import BannerPage from "../components/BannerPage.jsx";

class Dashboard extends Component {

    componentDidMount() {
        this.props.fetchDashboard();
        this.props.fetchBanners();
    }

    render() {
        let banners;
        let cards;

        // FETCH_BANNERS_START
        if (this.props.banners.fetching) {
            banners = <div className="banner-wrapper"></div>;
        }

        // FETCH_BANNERS_OK
        if (this.props.banners.fetched && !this.props.dashboard.error) {
            banners = <BannerPage>{this.props.banners.items.map((banner, i) => <Banner key={i} banner={banner}/>)}</BannerPage>
        }

        // FETCH_DASHBOARD_START
        if (this.props.dashboard.fetching) {
            cards = <div className="spinner"></div>;
        }

        // FETCH_DASHBOARD_ERR
        if (this.props.dashboard.fetched && this.props.dashboard.error) {
            cards = (
                <div className="empty">
                    <FontAwesomeIcon icon="question-circle"/>
                    <p>FAILED TO LOAD</p>
                </div>
            );
        }

        // FETCH_DASHBOARD_OK
        if (this.props.dashboard.fetched && !this.props.dashboard.error) {
            cards = <CardList cards={this.props.dashboard.sections}/>;
        }

        return (
            <main>
                {banners}
                {cards}
            </main>
        );
    }
}

const mapStateToProps = (state) => ({
    dashboard: state.dashboard,
    banners: state.banners
});

const mapActionstoProps = {
    fetchDashboard,
    fetchBanners
};

export default connect(mapStateToProps, mapActionstoProps)(Dashboard);
