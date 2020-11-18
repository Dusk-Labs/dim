import React, { Fragment, useEffect } from "react";

import Banners from "../Components/Banners/Index";
import CardList from "../Components/CardList";

function Dashboard() {
  useEffect(() => {
    document.title = "Dim - Dashboard";
  }, [])

  return (
    <Fragment>
      <Banners/>
      <CardList path={`//${window.host}:8000/api/v1/dashboard`}/>
    </Fragment>
  );
};

export default Dashboard;