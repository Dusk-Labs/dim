import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import Toggle from "../../../Components/Toggle";

function Authentication() {
  const dispatch = useDispatch();

  const settings = useSelector(store => store.settings);

  const [disableAuth, setDisableAuth] = useState(false);

  useEffect(() => {
    const { data } = settings.globalSettings;

    setDisableAuth(data.disable_auth);
  }, [settings]);

  return (
    <section>
      <h2>Authentication</h2>
      <Toggle
        state={!disableAuth}
        name="Require a valid auth token for each request to the server."
      />
    </section>
  );
}

export default Authentication;
