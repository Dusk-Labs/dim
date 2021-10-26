import { cloneElement, useCallback, useEffect, useState } from "react";
import Modal from "react-modal";
import { NavLink } from "react-router-dom";

import LogoutBtn from "../Pages/Preferences/LogoutBtn";
import WrenchIcon from "../assets/Icons/Wrench";

import "./AccountActions.scss";

const AccountActions = (props) => {
  const [visible, setVisible] = useState(false);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? document.body.style.overflow = "hidden"
      : document.body.style.overflow = "unset";
  }, [visible]);

  const close = useCallback(() => {
    setVisible(false);

    if (props.cleanUp) {
      props.cleanUp();
    }
  }, [props]);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  console.log(props.children);

  return (
    <div className="modalBoxContainer">
      {props.children && (
        cloneElement(props.children, { onClick: () => open() })
      )}
      <Modal
        isOpen={visible}
        className="modalBox"
        id={props.id}
        onRequestClose={close}
        overlayClassName="popup"
      >
        <div className="modalAccountActions">
          <NavLink className="item" to="/preferences" onClick={close}>
            <WrenchIcon/>
            <h3>{"Preferences"}</h3>
          </NavLink>
          <div className="separator"/>
          <LogoutBtn/>
        </div>
      </Modal>
    </div>
  );
};

export default AccountActions;
