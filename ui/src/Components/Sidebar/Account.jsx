import React from "react";
import { NavLink } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import LogoutBtn from "./LogoutBtn.jsx";

const Account = () => (
  <section className="your-account">
    <header>
      <h4>Account</h4>
    </header>
    <div className="list">
      <NavLink to="/preferences" className="item">
        <FontAwesomeIcon icon="wrench"/>
        <p>Preferences</p>
      </NavLink>
      <LogoutBtn/>
    </div>
  </section>
);

export default Account;
