import { useEffect } from "react";

import Banners from "./Banners/Index";
import CardList from "./CardList/Index";

function Dashboard() {
  useEffect(() => {
    document.title = "Dim - Dashboard";
  }, []);

  return (
    <div className="dashboard">
      <Banners/>
      <CardList path={"/api/v1/dashboard"}/>
    </div>
  );
}

export default Dashboard;
