import { useEffect, useRef } from "react";
import { useDispatch } from "react-redux";

import { fetchUser } from "../../actions/user.js";
import { fetchUserSettings } from "../../actions/settings.js";

import Profile from "./Profile/Index";
import Search from "./Search";
import Libraries from "./Libraries";
import Toggle from "./Toggle";
import Account from "./Account";

import "./Index.scss";

function Sidebar() {
  const dispatch = useDispatch();
  const divContainer = useRef(null);

  useEffect(() => {
    dispatch(fetchUser());
    dispatch(fetchUserSettings());
  }, [dispatch]);

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

export default Sidebar;
