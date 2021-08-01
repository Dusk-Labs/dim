import { useCallback } from "react";
import Toggle from "../../../Components/Toggle";

import "./Index.scss";

function VideoPlayer() {
  const toggleShowSubsDefault = useCallback((state) => {
    console.log(state);
  }, []);

  return (
    <div className="preferencesVideoPlayer">
      <section>
        <h2>Subtitles</h2>
        <Toggle
          disabled
          name="Show subtitles by default"
          onToggle={toggleShowSubsDefault}
        />
      </section>
      <section>
        <h2>Audio</h2>
        {/* <p className="desc">Manage your default preferred subtitle language</p> */}
      </section>
    </div>
  );
}

export default VideoPlayer;
