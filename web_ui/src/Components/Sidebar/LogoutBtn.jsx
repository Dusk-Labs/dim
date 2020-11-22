import React, { useState, useCallback } from "react";
import { useHistory } from "react-router-dom";
import { connect } from "react-redux";
import Modal from "react-modal";

import { logout } from "../../actions/auth.js";
import Icon from "./Icon.jsx";

function LogoutBtn(props) {
  const history = useHistory();

  const [visible, setVisible] = useState(false);

  const close = useCallback(() => {
    setVisible(false);
  }, []);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  const confirm = useCallback(() => {
    props.logout();
    history.push("/login");
  }, []);

  return (
    <div className="item-wrapper">
      <a className="logout" onClick={open}>
        <Icon icon="logout"/>
        <p className="item-wrapper-name logout">Logout</p>
      </a>
      <Modal
        isOpen={visible}
        contentLabel="logout"
        className="confirmationBox"
        onRequestClose={close}
        overlayClassName="popupOverlay"
      >
        <h3>Confirm action</h3>
        <div className="separator"/>
        <p>Are you sure you want to logout?</p>
        <div className="options">
          <button className="confirmationBoxCancel" onClick={close}>Cancel</button>
          <button className="confirmationBoxContinue" onClick={confirm}>Logout</button>
        </div>
      </Modal>
    </div>
  );
};

const mapStateToProps = () => ({});

const mapActionsToProps = {
  logout
};

export default connect(mapStateToProps, mapActionsToProps)(LogoutBtn);
