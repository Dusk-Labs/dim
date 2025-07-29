import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";
import { updateGlobalSettings } from "../../../actions/settings";

import Toggle from "../../../Components/Toggle";

function TranscoderDefaults() {
  const dispatch = useDispatch();

  const enable_hwaccel = useSelector((store) => {
    const { data } = store.settings.globalSettings;

    return data.enable_hwaccel;
  });

  const handleToggle = useCallback(
    (state) => {
      dispatch(
        updateGlobalSettings({
          enable_hwaccel: state,
        })
      );
    },
    [dispatch]
  );

  return (
    <section>
      <h2>Transcoder Settings</h2>
      <Toggle
        onToggle={handleToggle}
        state={enable_hwaccel}
        name="Enable hardware acceleration (if available)."
      />
    </section>
  );
}

export default TranscoderDefaults;
