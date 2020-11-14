import React from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";

import { logout } from "../../actions/auth.js";

import Icon from "./Icon.jsx";

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
      <div className="item-wrapper">
        <NavLink className="logout" to="/login" onClick={props.logout}>
          <Icon icon="logout"/>
          <p className="item-wrapper-name logout">Logout</p>
        </NavLink>
      </div>
    </div>
  </section>
);

const mapStateToProps = () => ({});

const mapActionsToProps = {
  logout
};

export default connect(mapStateToProps, mapActionsToProps)(Account);
