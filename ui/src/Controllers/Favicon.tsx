import { useEffect } from "react";

function FaviconController() {
  /*
    true: white logo (dark mode)
    false: black logo (light mode)
  */
  const updateLogo = (color: boolean) => {
    const favicon = document.getElementById("favicon");
    const textFavicon = document.getElementById("textFavicon");

    if (
      !(favicon instanceof HTMLLinkElement) ||
      !(textFavicon instanceof HTMLLinkElement)
    ) {
      return;
    }

    if (color) {
      favicon.href = "/static/logoWhite128.png";
      textFavicon.href = "/static/textLogoWhite128.png";
    } else {
      favicon.href = "/static/logoBlack128.png";
      textFavicon.href = "/static/textLogoBlack128.png";
    }
  };

  useEffect(() => {
    const mql = matchMedia("(prefers-color-scheme: dark)");

    updateLogo(mql.matches);

    mql.addEventListener("change", (e) => updateLogo(e.matches));

    return () => {
      mql.removeEventListener("change", (e) => updateLogo(e.matches));
    };
  }, []);

  return null;
}

export default FaviconController;
