import React from "react";
import { useParams } from "react-router-dom";

import Actions from "./Actions";
import CardList from "../../Components/CardList/Index";

const Library = () => (
  <div className="library">
    <CardList path={`//${window.host}:8000/api/v1/library/${useParams().id}/media`}/>
    <Actions id={useParams().id}/>
  </div>
);

export default Library;