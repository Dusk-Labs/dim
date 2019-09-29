import React, { Component } from "react";

import Library from "./Library.jsx";
import Banner from "../components/dashboard/Banner.jsx";
import BannerPage from "../components/dashboard/BannerPage.jsx";

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
                    {this.state.banners.map((banner, i) => <Banner key={i} banner={banner}/>)}
                </BannerPage>
                )}
                <Library path="http://86.21.150.167:8000/api/v1/dashboard"/>
            </main>
        );
    }
}

export default Dashboard;
