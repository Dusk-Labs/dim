import React from "react";
import { NavLink } from "react-router-dom";

import WrenchIcon from "../../assets/Icons/Wrench";

const Account = () => (
  <section className="your-account">
    <header>
      <h4>Account</h4>
    </header>
    <div className="list">
      <NavLink to="/preferences" className="item">
        <WrenchIcon/>
        <p>Preferences</p>
      </NavLink>
    </div>
  </section>
);

export default Account;
