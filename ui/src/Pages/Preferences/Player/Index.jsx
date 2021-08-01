import Subtitles from "./Subtitles";
import Audio from "./Audio";

import "./Index.scss";

const Player = () => (
  <div className="preferencesPlayer">
    <Subtitles/>
    <Audio/>
  </div>
);

export default Player;
