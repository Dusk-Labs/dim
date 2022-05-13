import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";
import { updateGlobalSettings } from "../../../actions/settings";

import Toggle from "../../../Components/Toggle";

function Authentication() {
  const dispatch = useDispatch();

  const { disableAuth } = useSelector((store) => {
    const { data } = store.settings.globalSettings;

    return {
      disableAuth: data.disable_auth,
    };
  });
  
  const { forwardAuth } = useSelector((store) => {
    const { data } = store.settings.globalSettings;

    return {
      disableAuth: data.forwarded_user_auth,
    };
  });

  const handleAuthToggle = useCallback(
    (state) => {
      dispatch(
        updateGlobalSettings({
          disable_auth: !state,
        })
      );
    },
    [dispatch]
  );

  const handleForwardAuthToggle = useCallback(
    (state) => {
      dispatch(
        updateGlobalSettings({
          forwarded_user_auth: state,
        })
      );
    },
    [dispatch]
  );

  return (
    <section>
      <h2>Authentication</h2>
      <Toggle
        onToggle={handleAuthToggle}
        state={!disableAuth}
        name="Require a valid auth token for each request to the server."
      />
      <Toggle
        onToggle={handleForwardAuthToggle}
        state={forwardAuth}
        name="Login users with the X-Forwarded-User header.  (Only do this if you're behind a reverse proxy that authenticates users.)"
      />
    </section>
  );
}

export default Authentication;
