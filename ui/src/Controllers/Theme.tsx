import { useEffect } from "react";

import { useAppSelector } from "hooks/store";
import DefaultTheme from "Themes/Default";
import BlindTheme from "Themes/Blind";
import LightsOff from "Themes/LightsOff";

function ThemeController() {
  const userSettings = useAppSelector((store) => store.settings.userSettings);

  useEffect(() => {
    switch (userSettings.data.theme) {
      case "Dark":
        for (const prop in DefaultTheme) {
          document.documentElement.style.setProperty(
            `--${prop}`,
            `${DefaultTheme[prop as keyof typeof DefaultTheme]}`
          );
        }
        break;
      case "Light":
        for (const prop in BlindTheme) {
          document.documentElement.style.setProperty(
            `--${prop}`,
            `${BlindTheme[prop as keyof typeof BlindTheme]}`
          );
        }
        break;
      case "Black":
        for (const prop in LightsOff) {
          document.documentElement.style.setProperty(
            `--${prop}`,
            `${LightsOff[prop as keyof typeof LightsOff]}`
          );
        }
        break;
      default:
        break;
    }
  }, [userSettings.data.theme]);

  return null;
}

export default ThemeController;
