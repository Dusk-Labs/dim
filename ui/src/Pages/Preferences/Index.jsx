import { createContext, useContext, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { checkAdminExists } from "../../actions/auth.js";
import { fetchGlobalSettings, fetchUserSettings } from "../../actions/settings.js";

import Account from "./Account/Index";
import Profile from "./Profile/Index.jsx";
import Invites from "./Invites/Index";
import Appearance from "./Appearance/Index";
import Player from "./Player/Index.jsx";
import Advanced from "./Advanced/Index";
import LogoutBtn from "./LogoutBtn.jsx";

import "./Index.scss";

const Context = createContext(null);

function Section(props) {
  const {active, setActive} = useContext(Context);

  return (
    <h3
      className={`${active === props.i && "active"}`}
      onClick={() => setActive(props.i)}
    >
      {props.children}
    </h3>
  );
}

function Preferences() {
  const dispatch = useDispatch();
  const auth = useSelector(store => store.auth);

  const [sections, setSections] = useState([]);
  const [active, setActive] = useState(0);

  useEffect(() => {
    const pages = [
      { name: "Account" },
      { name: "Profile" },
      { name: "Invites", show: auth.admin_exists },
      { name: "Appearance" },
      { name: "Player" },
      { name: "Advanced" }
    ];

    setSections(pages.filter(section => {
      if (section.show === undefined) return true;
      return section.show;
    }));
  }, [auth.admin_exists]);

  useEffect(() => {
    dispatch(checkAdminExists());
    dispatch(fetchUserSettings());
    dispatch(fetchGlobalSettings());
  }, [dispatch]);

  return (
    <div className="preferencesPage">
      <aside>
        <Context.Provider value={{active, setActive}}>
          {sections.map((section, i) => (
            <Section i={i}>{section.name}</Section>
          ))}
        </Context.Provider>
        <div className="separator"/>
        <LogoutBtn/>
      </aside>
      <div className="content">
        {active === 0 && <Account/>}
        {active === 1 && <Profile/>}
        {active === 2 && <Invites/>}
        {active === 3 && <Appearance/>}
        {active === 4 && <Player/>}
        {active === 5 && <Advanced/>}
      </div>
    </div>
  );
}

export default Preferences;
