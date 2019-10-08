import React, { Component } from "react";
import Card from "../components/library/Card.jsx";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import { connect } from "react-redux";
import { fetchDashboard } from "../actions/dashboardActions.js";
import { fetchBanners } from "../actions/dashboardActions.js";

import Banner from "../components/dashboard/Banner.jsx";
import BannerPage from "../components/dashboard/BannerPage.jsx";

class Dashboard extends Component {

    componentDidMount() {
        this.props.fetchDashboard();
        this.props.fetchBanners();
    }

    render() {
        let banners;
        let cards;

        // FETCH_BANNERS_OK
        if (this.props.banners.fetched && !this.props.dashboard.error) {
            banners = this.props.banners.items.map((banner, i) => <Banner key={i} banner={banner}/>);
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
            let { sections } = this.props.dashboard;

            // eslint-disable-next-line
            for (const section in sections) {
                if (sections[section].length > 0) {
                    const card_list = sections[section].map((card, i) => <Card key={i} data={card}/>);
                    sections[section] = card_list;
                }
            }

            cards = Object.keys(sections).map(section => {
                return (
                    <section key={section}>
                        <h1>{section}</h1>
                        <div className="cards">
                            { sections[section] }
                        </div>
                    </section>
                );
            });
        }

        return (
            <main>
                {this.props.banners.fetched && !this.props.banners.error &&
                    <BannerPage>{banners}</BannerPage>
                }
                <div className="library">{cards}</div>
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

