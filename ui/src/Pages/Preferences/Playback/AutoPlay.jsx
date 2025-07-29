import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";
import { updateUserSettings } from "../../../actions/settings";

import Toggle from "../../../Components/Toggle";

function Autoplay() {
  const dispatch = useDispatch();

  const enable_autoplay = useSelector((store) => {
    const { data } = store.settings.userSettings;

    return data.enable_autoplay;
  });

  const handleToggle = useCallback(
    (state) => {
      dispatch(
        updateUserSettings({
          enable_autoplay: state,
        })
      );
    },
    [dispatch]
  );

  return (
    <section>
      <h2>Autoplay next video</h2>
      <Toggle
        onToggle={handleToggle}
        state={enable_autoplay}
        name="If enabled, the next video will be played once the current video finishes."
      />
    </section>
  );
}

export default Autoplay;
