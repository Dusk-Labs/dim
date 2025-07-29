import { useEffect } from "react";

import Banners from "./Banners/Index";
import CardList from "./CardList/Index";

function Dashboard() {
  useEffect(() => {
    document.title = "Dim - Dashboard";
  }, []);

  return (
    <div className="dashboard">
      <Banners />
      <CardList />
    </div>
  );
}

export default Dashboard;
