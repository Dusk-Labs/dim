import React, { Component } from "react";

import Library from "./Library.jsx";
import Banner from "../components/dashboard/Banner.jsx";
import BannerPage from "../components/dashboard/BannerPage.jsx";

import "./Dashboard.scss";

class Dashboard extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: [],
            banners: [],
            bannersReady: false,
        };
    }

    componentDidMount() {
        this.fetchBanners();
    }

    fetchBanners = async () => {
        const bannersReq = await fetch("http://86.21.150.167:8000/api/v1/dashboard/banner");
        const banners = await bannersReq.json();
        this.setState({ banners, bannersReady: true });
    }

    render() {
        return (
            <main>
                { this.state.bannersReady && (
                <BannerPage>
                    {this.state.banners.map(({title, backdrop, synopsis, season, episode, duration, delta, banner_caption, genres, year}, i) => <Banner key={i} src={backdrop} title={title} description={synopsis} season={season} episode={episode} duration={duration} delta={delta === undefined ? 0 : delta} banner_caption={banner_caption} genres={genres} year={year}/>)}
                </BannerPage>
                )}
                <Library path="http://86.21.150.167:8000/api/v1/dashboard"/>
            </main>
        );
    }
}

export default Dashboard;
