import { useCallback } from "react";
import Toggle from "../../../Components/Toggle";

function Subtitles() {
  const toggleShowSubsDefault = useCallback((state) => {
    console.log(state);
  }, []);

  return (
    <section>
      <h2>Subtitles</h2>
      <Toggle
        disabled
        name="Show subtitles by default"
        onToggle={toggleShowSubsDefault}
      />
    </section>
  );
}

export default Subtitles;
