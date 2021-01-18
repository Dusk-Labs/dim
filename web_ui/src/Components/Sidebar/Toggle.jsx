import React, { useCallback, useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import DimLogo from "../../assets/DimLogo";

import "./Toggle.scss";

function Toggle(props) {
  const [visible, setVisible] = useState(true);

  const toggleSidebar = useCallback(() => {
    setVisible(state => !state);

    const main = document.querySelectorAll("main")[0];

    props.sidebar.current.classList.toggle("hide", visible);
    props.sidebar.current.classList.toggle("show", !visible);

    main.classList.toggle("full", visible);
    main.classList.toggle("shrunk", !visible);
  }, [visible]);

  useEffect(() => {
    if (window.innerWidth < 800) {
      toggleSidebar();
    }
  }, []);

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
