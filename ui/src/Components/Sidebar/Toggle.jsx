import React, { useCallback, useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import DimLogo from "../../assets/DimLogo";

import "./Toggle.scss";

function Toggle(props) {
  const [visible, setVisible] = useState(true);

  /*
    disabling animation is mainly intended for on-load layout prep changes
    e.g. hiding sidebar by default if not enough space detected.
  */
  const toggleSidebar = useCallback((withAnimation = true) => {
    setVisible(state => !state);

    const main = document.querySelectorAll("main")[0];

    if (withAnimation) {
      main.style.transition = "margin .3s ease-in-out";

      visible
        ? props.sidebar.current.style.animation = "hideSidebar .3s ease-in-out forwards"
        : props.sidebar.current.style.animation = "showSidebar .3s ease-in-out forwards";

      localStorage.setItem("defaultSidebarVisible", !visible);
    } else {
      main.style.transition = "";
      props.sidebar.current.style.animation = "";

      visible
        ? props.sidebar.current.style.transform = "translateX(-100%)"
        : props.sidebar.current.style.transform = "translateX(0)";
    }

    props.sidebar.current.classList.toggle("hide", visible);
    props.sidebar.current.classList.toggle("show", !visible);

    main.classList.toggle("full", visible);
    main.classList.toggle("shrunk", !visible);
  }, [props.sidebar, visible]);

  useEffect(() => {
    if (window.innerWidth < 800) {
      toggleSidebar(false);
      return;
    }

    const defaultSidebarVisible = localStorage.getItem("defaultSidebarVisible");

    if (defaultSidebarVisible === "false") {
      toggleSidebar(false);
    }
  }, [toggleSidebar]);

  return (
    <section className="sidebarToggleWrapper">
      <DimLogo/>
      <div className="toggle" onClick={toggleSidebar}>
          <FontAwesomeIcon icon="angle-left"/>
      </div>
    </section>
  );
}

export default Toggle;
