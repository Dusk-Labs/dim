import DefaultTheme from "../../assets/themes/Default";
import Blind from "../../assets/themes/Blind";
import LightsOff from "../../assets/themes/LightsOff";

import "./Appearance.scss";

function Appearance() {
  return (
    <div className="preferencesAppearance">
      <section>
        <h2>Themes</h2>
        <p className="desc">Change the look and feel of Dim across all your devices.</p>
        <div className="themes">
          <div className="themeContainer">
            <div className="theme active">
              <DefaultTheme/>
            </div>
            <p>Default</p>
          </div>
          <div className="themeContainer">
            <div className="theme">
              <Blind/>
            </div>
            <p>Blind</p>
          </div>
          <div className="themeContainer">
            <div className="theme">
              <LightsOff/>
            </div>
            <p>Lights Off</p>
          </div>
        </div>
      </section>
    </div>
  );
}

export default Appearance;
