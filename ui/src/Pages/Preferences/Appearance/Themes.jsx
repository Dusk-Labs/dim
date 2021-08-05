import DefaultTheme from "../../../assets/themes/Default";
import Blind from "../../../assets/themes/Blind";
import LightsOff from "../../../assets/themes/LightsOff";
import { useDispatch, useSelector } from "react-redux";
import { useCallback } from "react";
import { updateUserSettings } from "../../../actions/settings";

function Themes() {
  const dispatch = useDispatch();
  const userSettings = useSelector(store => store.settings.userSettings);

  const setTheme = useCallback((theme) => {
    dispatch(updateUserSettings({
      theme
    }));
  }, [dispatch]);

  return (
    <section>
      <h2>Themes</h2>
      <p className="desc">Change the look and feel of Dim across all your devices.</p>
      <div className="themes">
        <div className="themeContainer" onClick={() => setTheme("Dark")}>
          <div className={`theme ${userSettings.data.theme === "Dark" ? "active" : ""}`}>
            <DefaultTheme/>
          </div>
          <p>Dark</p>
        </div>
        <div className="themeContainer" onClick={() => setTheme("Blind")}>
          <div className={`theme ${userSettings.data.theme === "Blind" ? "active" : ""}`}>
            <Blind/>
          </div>
          <p>Blind</p>
        </div>
        <div className="themeContainer" onClick={() => setTheme("LightsOff")}>
          <div className={`theme ${userSettings.data.theme === "LightsOff" ? "active" : ""}`}>
            <LightsOff/>
          </div>
          <p>Lights Off</p>
        </div>
      </div>
    </section>
  );
}

export default Themes;
