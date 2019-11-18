import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchDashboard } from "../actions/dashboardActions.js";

import CardList from "../layouts/CardList.jsx";
import BannerPage from "../components/BannerPage.jsx";

class Dashboard extends Component {
    componentDidMount() {
        document.title = "Dim - Dashboard";
        this.props.fetchDashboard();
    }

    render() {
        let cards;

        // FETCH_DASHBOARD_START
        if (this.props.dashboard.fetching) {
            cards = <CardList placeholder={{failed: false}}/>;
        }

        // FETCH_DASHBOARD_ERR
        if (this.props.dashboard.fetched && this.props.dashboard.error) {
            cards = <CardList placeholder={{failed: true}}/>;
        }

        // FETCH_DASHBOARD_OK
        if (this.props.dashboard.fetched && !this.props.dashboard.error) {
            cards = <CardList cards={this.props.dashboard.sections}/>;
        }

        return (
            <main>
                <BannerPage/>
                {cards}
            </main>
        );
    }
}

const mapStateToProps = (state) => ({
    dashboard: state.dashboard
});

const mapActionstoProps = {
    fetchDashboard
};

export default connect(mapStateToProps, mapActionstoProps)(Dashboard);
