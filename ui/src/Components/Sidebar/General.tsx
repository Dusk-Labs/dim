import { NavLink } from "react-router-dom";

import HomeIcon from "../../assets/Icons/Home";

const General = () => (
  <section className="yourAccount">
    <header>
      <h4>General</h4>
    </header>
    <div className="list">
      <NavLink className="item" to="/" exact>
        <HomeIcon />
        <p>Dashboard</p>
      </NavLink>
    </div>
  </section>
);

export default General;
