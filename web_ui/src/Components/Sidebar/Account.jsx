import React from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";

import { logout } from "../../actions/auth.js";
import Icon from "./Icon.jsx";
import LogoutBtn from "./LogoutBtn.jsx";

const Account = (props) => (
  <section className="your-account">
    <header>
      <h4>Account</h4>
    </header>
    <div className="list">
      <div className="item-wrapper">
        <NavLink to="/preferences">
          <Icon icon="preferences"/>
          <p className="item-wrapper-name">Preferences</p>
        </NavLink>
      </div>
      <LogoutBtn/>
    </div>
  </section>
);

const mapStateToProps = () => ({});

const mapActionsToProps = {
  logout
};

export default connect(mapStateToProps, mapActionsToProps)(Account);
