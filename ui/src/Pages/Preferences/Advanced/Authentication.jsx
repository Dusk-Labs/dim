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

  const handleToggle = useCallback(
    (state) => {
      dispatch(
        updateGlobalSettings({
          disable_auth: !state,
        })
      );
    },
    [dispatch]
  );

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
