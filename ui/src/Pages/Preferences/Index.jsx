import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { checkAdminExists } from "../../actions/auth.js";
import { fetchGlobalSettings } from "../../actions/settings.js";

import Account from "./Account";
import Invites from "./Invites";
import Appearance from "./Appearance";
import Advanced from "./Advanced";

import "./Index.scss";

function Preferences() {
  const dispatch = useDispatch();

  const auth = useSelector(store => store.auth);

  const [active, setActive] = useState(0);

  useEffect(() => {
    dispatch(checkAdminExists());
    dispatch(fetchGlobalSettings());
  }, [dispatch]);

  return (
    <div className="preferencesPage">
      <aside>
        <h3 className={`${active === 0 && "active"}`} onClick={() => setActive(0)}>
          Account
        </h3>
        <h3 className={`${active === 1 && "active"}`} onClick={() => setActive(1)}>
          Profile
        </h3>
        {auth.admin_exists && (
          <h3 className={`${active === 2 && "active"}`} onClick={() => setActive(2)}>
            Invites
          </h3>
        )}
        <h3 className={`${active === 3 && "active"}`} onClick={() => setActive(3)}>
          Appearance
        </h3>
        <h3 className={`${active === 4 && "active"}`} onClick={() => setActive(4)}>
          Advanced
        </h3>
        <h3>Logout</h3>
      </aside>
      <div className="content">
        {active === 0 && <Account/>}
        {active === 2 && <Invites/>}
        {active === 3 && <Appearance/>}
        {active === 4 && <Advanced/>}
      </div>
    </div>
  );
}

export default Preferences;
