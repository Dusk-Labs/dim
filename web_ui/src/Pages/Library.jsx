import React from "react";

import MainLayout from "../Layouts/MainLayout.jsx";
import CardList from "../Components/CardList.jsx";

const Library = (props) => (
    <MainLayout>
        <CardList path={`//${window.host}:8000/api/v1/library/${props.match.params.id}/media`}/>
    </MainLayout>
);

export default Library;