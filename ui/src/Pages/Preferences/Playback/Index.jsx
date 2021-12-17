import Defaults from "./Defaults";
import TranscoderDefaults from "./TranscoderDefaults";
import EnableSsa from "./EnableSsa";

import "./Index.scss";

const Playback = () => (
  <div className="preferencesPlayback">
    <Defaults/>
    <TranscoderDefaults/>
    <EnableSsa/>
  </div>
);

export default Playback;
