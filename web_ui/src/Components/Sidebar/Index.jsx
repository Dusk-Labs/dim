import React, { useEffect, useRef } from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";

import { logout } from "../../actions/auth.js";
import { fetchUser } from "../../actions/user.js";

import Profile from "./Profile.jsx";
import Search from "./Search.jsx";
import Icon from "./Icon.jsx";
import Libraries from "./Libraries";
import Toggle from "./Toggle";

import "./Index.scss";

function Sidebar(props) {
  const divContainer = useRef(null);

  useEffect(() => {
    props.fetchUser(props.auth.token);
  }, []);

  return (
    <nav className="sidebar" ref={divContainer}>
      <div className="sectionsWrapper">
        <section className="main-part">
          <Profile name={true} hoursSpentWatching={true}/>
          <div className="separator"/>
          <Search/>
        </section>
        <Libraries/>
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
              <NavLink to="/login" onClick={props.logout}>
                <Icon icon="logout"/>
                <p className="item-wrapper-name">Logout</p>
              </NavLink>
            </div>
          </div>
        </section>
      </div>
      <Toggle sidebar={divContainer}/>
    </nav>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
});

const mapActionsToProps = {
  logout,
  fetchUser
};

export default connect(mapStateToProps, mapActionsToProps)(Sidebar);
