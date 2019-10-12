import React, { Component } from "react";
import { connect } from "react-redux";

import { fetchDashboard } from "../actions/dashboardActions.js";
import { fetchBanners } from "../actions/dashboardActions.js";

import CardList from "../layouts/CardList.jsx";
import Banner from "../components/dashboard/Banner.jsx";
import BannerPage from "../components/dashboard/BannerPage.jsx";

class Dashboard extends Component {

    componentDidMount() {
        this.props.fetchDashboard();
        this.props.fetchBanners();
    }

    render() {
        let banners;

        // FETCH_BANNERS_OK
        if (this.props.banners.fetched && !this.props.dashboard.error) {
            banners = this.props.banners.items.map((banner, i) => <Banner key={i} banner={banner}/>);
        }

        return (
            <main>
                {this.props.banners.fetched && !this.props.banners.error &&
                    <BannerPage>{banners}</BannerPage>
                }
                {/* FETCH_DASHBOARD_OK */}
                {this.props.dashboard.fetched && !this.props.dashboard.error &&
                    <CardList cards={this.props.dashboard.sections}/>
                }
            </main>
        );
    }
}

const mapStateToProps = (state) => ({
    dashboard: state.dashboard,
    banners: state.banners
});

const actions = {
    fetchDashboard,
    fetchBanners
};

export default connect(mapStateToProps, actions)(Dashboard);
