import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateUserSettings } from "../../../actions/settings";
import Toggle from "../../../Components/Toggle";

function Sidebar() {
  const dispatch = useDispatch();
  const settings = useSelector((store) => store.settings);

  const [compactSidebar, setCompactSidebar] = useState(false);

  useEffect(() => {
    setCompactSidebar(settings.userSettings.data.is_sidebar_compact);
  }, [settings.userSettings.data.is_sidebar_compact]);

  const handleToggle = useCallback(
    (state) => {
      dispatch(
        updateUserSettings({
          is_sidebar_compact: state,
        })
      );
    },
    [dispatch]
  );

  return (
    <section>
      <h2>Sidebar</h2>
      <Toggle
        onToggle={handleToggle}
        state={compactSidebar}
        name="Keep the sidebar always in compact mode"
      />
    </section>
  );
}

export default Sidebar;
