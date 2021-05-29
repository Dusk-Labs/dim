import { useCallback, useEffect, useState, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { checkAdminExists } from "../../actions/auth.js";

import Account from "./Account";
import Invites from "./Invites";
import FileBrowser from "./FileBrowser";
import Appearance from "./Appearance";
import Advanced from "./Advanced";

import "./Index.scss";

function Preferences(props) {
  const user = useSelector(store => store.user);
  const dispatch = useDispatch();

  const [active, setActive] = useState(1);

  const editBadge = useRef(null);
  const leftProfilePic = useRef(null);
  const badge = useRef(null);

  const [tabs, setTabs] = useState([]);
  const [badgePos, setBagePos] = useState({right: 0, top: 0});

  const tempStats = [
    {"name" : "Watched", val: "33h"},
    {"name" : "Users", val: "4"},
    {"name" : "Tokens", val: "3"}
  ];

  useEffect(() => {
    document.title = "Dim - Preferences";
  }, []);

  useEffect(() => {
    dispatch(checkAdminExists());
  }, [dispatch]);

  const switchTo = useCallback((i) => {
    setActive(i);
  }, []);

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
              <img className="leftBarProfileImg" src={user.info.picture}/>
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

        </div>
      </div>
    </div>
  );
}

export default Preferences;
