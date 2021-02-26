import React, { useEffect, useRef } from "react";
import { connect } from "react-redux";

import { fetchUser } from "../../actions/user.js";

import Profile from "./Profile/Index";
import Search from "./Search";
import Libraries from "./Libraries";
import Toggle from "./Toggle";
import Account from "./Account";

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
        <Account/>
      </div>
      <Toggle sidebar={divContainer}/>
    </nav>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
});

const mapActionsToProps = {
  fetchUser
};

export default connect(mapStateToProps, mapActionsToProps)(Sidebar);
