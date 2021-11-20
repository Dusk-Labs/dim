import { useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { Helmet } from "react-helmet";

import DefaultTheme from "../Themes/Default";
import BlindTheme from "../Themes/Blind";
import LightsOff from "../Themes/LightsOff";

function ThemeController() {
  const userSettings = useSelector(store => store.settings.userSettings);
  const [themeColor, setThemeColor] = useState("#1a1a1a");

  useEffect(() => {
    switch(userSettings.data.theme) {
      case "Dark":
        for (const prop in DefaultTheme) {
          document.documentElement.style.setProperty(`--${prop}`, `${DefaultTheme[prop]}`);
        }
        setThemeColor(userSettings.data.theme.primaryColor);
        break;
      case "Light":
        for (const prop in BlindTheme) {
          document.documentElement.style.setProperty(`--${prop}`, `${BlindTheme[prop]}`);
        }
        setThemeColor(userSettings.data.theme.primaryColor);
        break;
      case "Black":
        for (const prop in LightsOff) {
          document.documentElement.style.setProperty(`--${prop}`, `${LightsOff[prop]}`);
        }
        setThemeColor(userSettings.data.theme.primaryColor);
        break;
      default:
        break;
    }
  }, [setThemeColor, userSettings.data.theme]);

  return (
    <Helmet>
      <meta name="theme-color" content={themeColor} />
    </Helmet>
  );
}

export default ThemeController;
