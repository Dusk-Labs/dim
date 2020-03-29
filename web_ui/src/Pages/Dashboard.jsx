import React from "react";

import MainLayout from "../Layouts/MainLayout.jsx";
import BannerPage from "../Components/BannerPage.jsx";
import CardList from "../Components/CardList.jsx";

const Dashboard = () => {
    document.title = "Dim - Dashboard";

    return (
        <MainLayout>
            <BannerPage/>
            <CardList path={`//${window.host}:8000/api/v1/dashboard`}/>
        </MainLayout>
    );
};

export default Dashboard;