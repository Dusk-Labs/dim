import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { updateGlobalSettings } from "../../../actions/settings";

import Toggle from "../../../Components/Toggle";

function Authentication() {
  const dispatch = useDispatch();
  const settings = useSelector(store => store.settings);

  const [disableAuth, setDisableAuth] = useState(false);

  useEffect(() => {
    const { data } = settings.globalSettings;
    setDisableAuth(data.disable_auth);
  }, [settings]);

  const handleToggle = useCallback((state) => {
    dispatch(updateGlobalSettings({
      disable_auth: !state
    }));
  }, [dispatch]);

  return (
    <section>
      <h2>Authentication</h2>
      <Toggle
        onToggle={handleToggle}
        state={!disableAuth}
        name="Require a valid auth token for each request to the server."
      />
    </section>
  );
}

export default Authentication;
