import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateUserSettings } from "../../../actions/settings";
import Toggle from "../../../Components/Toggle";

function Cards() {
  const dispatch = useDispatch();

  const { showHoverCards, showCardNames } = useSelector((store) => {
    const { data } = store.settings.userSettings;

    return {
      showHoverCards: data.show_hovercards,
      showCardNames: data.show_card_names,
    };
  });

  const handleShowMediaNamesToggle = useCallback(
    (state) => {
      dispatch(
        updateUserSettings({
          show_card_names: state,
        })
      );
    },
    [dispatch]
  );

  const handleShowHoverCardsToggle = useCallback(
    (state) => {
      dispatch(
        updateUserSettings({
          show_hovercards: state,
        })
      );
    },
    [dispatch]
  );

  return (
    <section>
      <h2>Cards</h2>
      <Toggle
        onToggle={handleShowMediaNamesToggle}
        state={showCardNames}
        name="Show media name under cards across the dashboard and libraries"
      />
      <Toggle
        onToggle={handleShowHoverCardsToggle}
        state={showHoverCards}
        name="Show media information when hovering over a card"
      />
    </section>
  );
}

export default Cards;
