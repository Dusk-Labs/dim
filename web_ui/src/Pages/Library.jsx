import React from "react";
import { useParams } from "react-router-dom";

import CardList from "../Components/CardList.jsx";

const Library = () => (
    <CardList path={`//${window.host}:8000/api/v1/library/${useParams().id}/media`}/>
);

export default Library;