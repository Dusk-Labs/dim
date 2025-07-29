import Defaults from "./Defaults";
import TranscoderDefaults from "./TranscoderDefaults";
import EnableSsa from "./EnableSsa";
import AutoPlay from "./AutoPlay";

import "./Index.scss";

const Playback = () => (
  <div className="preferencesPlayback">
    <Defaults />
    <TranscoderDefaults />
    <AutoPlay />
    <EnableSsa />
  </div>
);

export default Playback;
