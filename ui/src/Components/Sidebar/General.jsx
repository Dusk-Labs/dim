import { NavLink } from "react-router-dom";

import HomeIcon from "../../assets/Icons/Home";
import WrenchIcon from "../../assets/Icons/Wrench";

const General = () => (
  <section className="yourAccount">
    <header>
      <h4>General</h4>
    </header>
    <div className="list">
      <NavLink className="item" to="/" exact>
        <HomeIcon/>
        <p>Dashboard</p>
      </NavLink>
      <NavLink to="/preferences" className="item">
        <WrenchIcon/>
        <p>Preferences</p>
      </NavLink>
    </div>
  </section>
);

export default General;
