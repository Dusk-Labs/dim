import React, { useCallback, useEffect, useState } from "react";
import { useSelector } from "react-redux";

import { RootState } from "../../store";

import DimLogo from "assets/DimLogo";
import AngleLeftIcon from "assets/Icons/AngleLeft";

import "./Toggle.scss";

interface Props {
  sidebar: React.MutableRefObject<HTMLElement | null>;
}

function Toggle(props: Props) {
  const [defaultChecked, setDefaultChecked] = useState(false);
  const [visible, setVisible] = useState(true);

  const version = useSelector((store: RootState) => {
    const { data } = store.settings.globalSettings;
    return data.version;
  });

  /*
    disabling animation is mainly intended for on-load layout prep changes
    e.g. hiding sidebar by default if not enough space detected.
  */
  const toggleSidebar = useCallback(
    (withAnimation = true) => {
      setVisible((state) => !state);

      const main = document.querySelectorAll("main")[0];

      if (withAnimation) {
        main.style.transition = "margin .3s ease-in-out";

        if (props.sidebar.current) {
          if (visible) {
            props.sidebar.current.style.animation =
              "hideSidebar .3s ease-in-out forwards";
          } else {
            props.sidebar.current.style.animation =
              "showSidebar .3s ease-in-out forwards";
          }
        }

        localStorage.setItem("defaultSidebarVisible", (!visible).toString());
      } else {
        main.style.transition = "";

        if (props.sidebar.current) {
          props.sidebar.current.style.animation = "";

          if (visible) {
            props.sidebar.current.style.transform = "translateX(-100%)";
          } else {
            props.sidebar.current.style.transform = "translateX(0)";
          }
        }
      }

      if (props.sidebar.current) {
        props.sidebar.current.classList.toggle("hide", visible);
        props.sidebar.current.classList.toggle("show", !visible);
      }

      main.classList.toggle("full", visible);
      main.classList.toggle("shrunk", !visible);
    },
    [props.sidebar, visible]
  );

  useEffect(() => {
    if (defaultChecked) return;

    if (window.innerWidth < 800) {
      toggleSidebar(false);
      setDefaultChecked(true);
      return;
    }

    const defaultSidebarVisible = localStorage.getItem("defaultSidebarVisible");

    if (defaultSidebarVisible === "false") {
      toggleSidebar(false);
    }

    setDefaultChecked(true);
  }, [defaultChecked, toggleSidebar]);

  return (
    <section className="sidebarToggleWrapper">
      <DimLogo />
      <span className="version">{version}</span>
      <div className="toggle" onClick={toggleSidebar}>
        <AngleLeftIcon />
      </div>
    </section>
  );
}

export default Toggle;
