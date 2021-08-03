import { useEffect } from "react";
import { useSelector } from "react-redux";

import DefaultTheme from "../Themes/Default";
import BlindTheme from "../Themes/Blind";

function ThemeController() {
  const userSettings = useSelector(store => store.settings.userSettings);

  useEffect(() => {
    switch(userSettings.data.theme) {
      case "Dark":
        for (const prop in BlindTheme) {
          document.documentElement.style.setProperty(`--${prop}`, `${DefaultTheme[prop]}`);
        }
        break;
      case "Blind":
        for (const prop in BlindTheme) {
          document.documentElement.style.setProperty(`--${prop}`, `${BlindTheme[prop]}`);
        }
        break;
      default:
        break;
    }
  }, [userSettings.data.theme]);

  return null;
}

export default ThemeController;
