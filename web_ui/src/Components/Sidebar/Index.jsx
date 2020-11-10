import React, { useCallback, useEffect, useRef, useState } from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { logout } from "../../actions/auth.js";
import { fetchUser } from "../../actions/user.js";

import Profile from "./Profile.jsx";
import Search from "./Search.jsx";
import Icon from "./Icon.jsx";
import Libraries from "./Libraries";

import "./Index.scss";

function Sidebar(props) {
  const divContainer = useRef(null);
  const [visible, setVisible] = useState(true);

  const toggleSidebar = useCallback(() => {
    setVisible(state => !state);

    const main = document.querySelectorAll("main")[0];

    divContainer.current.classList.toggle("hide", visible);
    divContainer.current.classList.toggle("show", !visible);

    main.classList.toggle("full", visible);
    main.classList.toggle("shrunk", !visible);
  }, [visible]);

  useEffect(() => {
    if (window.innerWidth < 800) {
      toggleSidebar();
    }

    props.fetchUser(props.auth.token);
  }, []);

  return (
    <nav className="sidebar" ref={divContainer}>
      <div className="toggle" onClick={toggleSidebar}>
        <FontAwesomeIcon icon="angle-left"/>
      </div>
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
