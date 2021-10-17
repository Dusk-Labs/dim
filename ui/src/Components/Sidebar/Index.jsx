import { useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchUser } from "../../actions/user.js";
import { fetchUserSettings } from "../../actions/settings.js";
import { fetchLibraries } from "../../actions/library.js";
import { logout } from "../../actions/auth.js";

import Profile from "./Profile/Index";
import Search from "./Search";
import Libraries from "./Libraries";
import Toggle from "./Toggle";
import General from "./General";

import "./Index.scss";

function Sidebar() {
  const user = useSelector(store => store.user);
  const dispatch = useDispatch();
  const divContainer = useRef(null);

  useEffect(() => {
    dispatch(fetchLibraries());
    dispatch(fetchUser());
    dispatch(fetchUserSettings());
  }, [dispatch]);

  useEffect(() => {
    if(user.error === "Unauthorized") {
      console.log("[auth] token expired, logging out.");
      dispatch(logout());
    }
  }, [dispatch, user.error]);

  return (
    <nav className="sidebar" ref={divContainer}>
      <div className="sectionsWrapper">
        <section className="main-part">
          <Profile name={true} hoursSpentWatching={true}/>
          <div className="separator"/>
          <Search/>
        </section>
        <Libraries/>
        <General/>
      </div>
      <Toggle sidebar={divContainer}/>
    </nav>
  );
}

export default Sidebar;
