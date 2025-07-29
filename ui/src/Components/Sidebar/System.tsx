import { NavLink } from "react-router-dom";

import WrenchIcon from "../../assets/Icons/Wrench";

const General = () => (
  <section className="yourAccount">
    <header>
      <h4>System</h4>
    </header>
    <div className="list">
      <NavLink to="/preferences" className="item">
        <WrenchIcon />
        <p>Preferences</p>
      </NavLink>
    </div>
  </section>
);

export default General;
