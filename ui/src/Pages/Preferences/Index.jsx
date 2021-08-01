import { createContext, useContext, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchGlobalSettings, fetchUserSettings } from "../../actions/settings.js";

import Account from "./Account/Index";
import Profile from "./Profile/Index";
import Invites from "./Invites/Index";
import Appearance from "./Appearance/Index";
import Player from "./Player/Index";
import Advanced from "./Advanced/Index";
import LogoutBtn from "./LogoutBtn";

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
  const user = useSelector(store => store.user);

  const [sections, setSections] = useState([]);
  const [active, setActive] = useState(0);

  useEffect(() => {
    if (!user.fetched && !user.error) return;

    const pages = [
      { name: "Account" },
      { name: "Profile" },
      { name: "Invites", show: user.info.roles.includes("owner") },
      { name: "Appearance" },
      { name: "Player" },
      { name: "Advanced" }
    ];

    setSections(pages);
  }, [user.error, user.fetched, user.info.roles]);

  useEffect(() => {
    dispatch(fetchUserSettings());
    dispatch(fetchGlobalSettings());
  }, [dispatch]);

  return (
    <div className="preferencesPage">
      <aside>
        <Context.Provider value={{active, setActive}}>
          {sections.map((section, i) => (
            (section.show === true || section.show === undefined)
              ? <Section i={i} key={i}>{section.name}</Section>
              : <></>
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
