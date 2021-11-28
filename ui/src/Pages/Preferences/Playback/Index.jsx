import Defaults from "./Defaults";
import TranscoderDefaults from "./TranscoderDefaults";

import "./Index.scss";

const Playback = () => (
  <div className="preferencesPlayback">
    <Defaults/>
    <TranscoderDefaults/>
  </div>
);

export default Playback;
