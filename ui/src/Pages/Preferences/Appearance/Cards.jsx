import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateUserSettings } from "../../../actions/settings";
import Toggle from "../../../Components/Toggle";

function Cards() {
  const dispatch = useDispatch();
  const settings = useSelector(store => store.settings);

  const [showMediaNames, setShowMediaNames] = useState(false);

  useEffect(() => {
    setShowMediaNames(settings.userSettings.data.show_card_names);
  }, [settings.userSettings.data.show_card_names]);

  const handleToggle = useCallback((state) => {
    dispatch(updateUserSettings({
      show_card_names: state
    }));
  }, [dispatch]);

  return (
    <section>
      <h2>Cards</h2>
      <Toggle
        onToggle={handleToggle}
        state={showMediaNames}
        name="Show media name under cards across the dashboard and libraries"
      />
    </section>
  );
}

export default Cards;
