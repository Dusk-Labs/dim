import { useEffect, useRef } from "react";
import { useDispatch} from "react-redux";

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

  useEffect(() => {
    dispatch(fetchLibraries());
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
        <General/>
      </div>
      <Toggle sidebar={divContainer}/>
    </nav>
  );
}

export default Sidebar;
