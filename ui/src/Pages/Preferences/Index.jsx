import { useEffect, useState, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { checkAdminExists } from "../../actions/auth.js";
import { fetchGlobalSettings } from "../../actions/settings.js";

import Account from "./Account";
// import Invites from "./Invites";
// import FileBrowser from "./FileBrowser";
import Appearance from "./Appearance";
import Advanced from "./Advanced";

import "./Index.scss";

function Preferences(props) {
  const {user, settings} = useSelector(store => {
    return {user: store.user, settings: store.settings};
  });
  const dispatch = useDispatch();

  const [active, setActive] = useState(0);

  const editBadge = useRef(null);
  const leftProfilePic = useRef(null);
  const badge = useRef(null);

  const [badgePos, setBagePos] = useState({right: 0, top: 0});

  const tempStats = [
    {"name" : "Watched", val: "33h"},
    {"name" : "Users", val: "4"},
    {"name" : "Tokens", val: "3"}
  ];

  useEffect(() => {
    dispatch(checkAdminExists());
    dispatch(fetchGlobalSettings());
  }, [dispatch]);

  useEffect(() => {
    console.log(settings);
  }, [settings]);

  function computeCirclePos() {
    let containerHeight = leftProfilePic.current.clientHeight;
    let containerRadius = leftProfilePic.current.clientWidth / 2;
    let badgeWidth = editBadge.current.clientWidth;
    let smallBadgeWidth = badge.current.clientWidth;

    setBagePos({
      top: (containerHeight / 2) - badgeWidth / 2,
      left: (containerRadius) + badgeWidth - smallBadgeWidth,
      width: containerRadius + smallBadgeWidth / 2
    });
  }

  return (
    <div className="preferencesPage">
      <div className="preferences">
        <div className="leftBar">
          <div className="leftBarImgContainer">
            <div ref={leftProfilePic} class="leftBarImgParent">
              <img alt="ProfilePic" className="leftBarProfileImg" onLoad={computeCirclePos} src={user.info.picture}/>
              <div className="circle" style={badgePos} ref={editBadge}><div ref={badge} className="leftBarImgEdit"/></div>
            </div>
          </div>
          <div className="leftBarNames">
            <div className="leftBarUsername">
              {user.info.username}
            </div>
            <div className="leftBarRole">
              {"Admin"}
            </div>
          </div>
          <div className="leftBarStatistics">
            {tempStats.map((stat, i) => (
              <div key={i} className="leftBarStat">
                <div className="leftBarStatValue">{stat.val}</div>
                <div className="leftBarStatName">{stat.name}</div>
              </div>
            ))}
          </div>
          <hr className="leftBarSep"/>
          <div className="leftBarTabs">
            <div className={active === 0 && "active"} onClick={() => setActive(0)}>Account</div>
            <div className={active === 1 && "active"} onClick={() => setActive(1)}>Appearance</div>
            <div className={active === 2 && "active"} onClick={() => setActive(2)}>Advanced</div>
          </div>
        </div>
        <div className="content">
          {active === 0 && <Account/>}
          {active === 1 && <Appearance/>}
          {active === 2 && <Advanced/>}
        </div>
      </div>
    </div>
  );
}

export default Preferences;
