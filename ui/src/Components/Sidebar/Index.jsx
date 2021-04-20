import { useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchUser } from "../../actions/user.js";

import Profile from "./Profile/Index";
import Search from "./Search";
import Libraries from "./Libraries";
import Toggle from "./Toggle";
import Account from "./Account";

import "./Index.scss";

function Sidebar() {
  const dispatch = useDispatch();

  const auth = useSelector(store => store.auth);

  const divContainer = useRef(null);

  useEffect(() => {
    dispatch(fetchUser(auth.token));
  }, [auth.token, dispatch]);

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
