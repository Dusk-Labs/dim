import { useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchUser } from "../../actions/user.js";
import { fetchUserSettings } from "../../actions/settings.js";
import { fetchLibraries } from "../../actions/library.js";

import Profile from "./Profile/Index";
import Search from "./Search";
import Libraries from "./Libraries";
import Toggle from "./Toggle";
import General from "./General";

import "./Index.scss";

function Sidebar() {
  const dispatch = useDispatch();
  const divContainer = useRef(null);

  const libraries = useSelector(store => (
    store.library.fetch_libraries
  ));

  useEffect(() => {
    dispatch(fetchLibraries());
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
        {libraries.items.length > 0 && (
          <Libraries/>
        )}
        <General/>
      </div>
      <Toggle sidebar={divContainer}/>
    </nav>
  );
}

export default Sidebar;
