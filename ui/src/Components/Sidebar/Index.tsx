import { useEffect, useRef } from "react";

import { useAppDispatch } from "hooks/store";
import { fetchGlobalSettings, fetchUserSettings } from "actions/settings.js";
import { fetchLibraries } from "actions/library.js";

import Profile from "./Profile/Index";
import Search from "./Search";
import Libraries from "./Libraries";
import Toggle from "./Toggle";
import General from "./General";
import System from "./System";

import "./Index.scss";

function Sidebar() {
  const dispatch = useAppDispatch();
  const divContainer = useRef(null);

  useEffect(() => {
    dispatch(fetchLibraries());
    dispatch(fetchUserSettings());
    dispatch(fetchGlobalSettings());
  }, [dispatch]);

  return (
    <nav className="sidebar" ref={divContainer}>
      <div className="sectionsWrapper">
        <section className="main-part">
          <Profile hoursSpentWatching={true} />
          <div className="separator" />
          <Search />
        </section>
        <General />
        <Libraries />
        <System />
      </div>
      <Toggle sidebar={divContainer} />
    </nav>
  );
}

export default Sidebar;
