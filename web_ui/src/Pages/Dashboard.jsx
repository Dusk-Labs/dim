import React, { Fragment } from "react";

import BannerPage from "../Components/BannerPage.jsx";
import CardList from "../Components/CardList.jsx";

const Dashboard = () => {
    document.title = "Dim - Dashboard";

    return (
        <Fragment>
            <BannerPage/>
            <CardList path={`//${window.host}:8000/api/v1/dashboard`}/>
        </Fragment>
    );
};

export default Dashboard;