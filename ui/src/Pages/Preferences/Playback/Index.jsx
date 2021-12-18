import Defaults from "./Defaults";
import TranscoderDefaults from "./TranscoderDefaults";
import AutoPlay from "./AutoPlay";

import "./Index.scss";

const Playback = () => (
  <div className="preferencesPlayback">
    <Defaults/>
    <TranscoderDefaults/>
    <AutoPlay/>
  </div>
);

export default Playback;
