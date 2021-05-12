import { useCallback, useEffect, useState } from "react";
import { useDispatch } from "react-redux";

import { checkAdminExists } from "../../actions/auth.js";

import Account from "./Account";
import Invites from "./Invites";
import FileBrowser from "./FileBrowser";
import Appearance from "./Appearance";
import Advanced from "./Advanced";

import "./Index.scss";

function Preferences() {
  const dispatch = useDispatch();

  const [active, setActive] = useState(1);

  useEffect(() => {
    document.title = "Dim - Preferences";
  }, []);

  useEffect(() => {
    dispatch(checkAdminExists());
  }, [dispatch]);

  const switchTo = useCallback((i) => {
    setActive(i);
  }, []);

  return (
    <div className="preferencesPage">
      <nav>
        <h1>Preferences</h1>
        <div className="items">
          <p
            className={`item ${active === 0 ? "active" : ""}`}
            onClick={() => switchTo(0)}
          >
            Account
          </p>
          <p
            className={`item ${active === 1 ? "active" : ""}`}
            onClick={() => switchTo(1)}
          >
            Invites
          </p>
          <p
            className={`item ${active === 2 ? "active" : ""}`}
            onClick={() => switchTo(2)}
          >
            File Browser
          </p>
          <p
            className={`item ${active === 3 ? "active" : ""}`}
            onClick={() => switchTo(3)}
          >
            Appearance
          </p>
          <p
            className={`item ${active === 4 ? "active" : ""}`}
            onClick={() => switchTo(4)}
          >
            Advanced
          </p>
        </div>
      </nav>
      <div className="content">
        {active === 0 && <Account/>}
        {active === 1 && <Invites/>}
        {active === 2 && <FileBrowser/>}
        {active === 3 && <Appearance/>}
        {active === 4 && <Advanced/>}
      </div>
    </div>
  );
}

export default Preferences;
