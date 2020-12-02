import React, { useState, useCallback, useEffect } from "react";
import { useHistory } from "react-router-dom";
import { connect } from "react-redux";
import Modal from "react-modal";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { logout } from "../../actions/auth.js";

import "./LogoutBtn.scss";

function LogoutBtn(props) {
  const history = useHistory();

  const [visible, setVisible] = useState(false);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? document.body.style.overflow = 'hidden'
      : document.body.style.overflow = 'unset';
  }, [visible]);

  const close = useCallback(() => {
    console.log('testing')
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
    <>
      <a className="item logout" onClick={open}>
        <FontAwesomeIcon icon="sign-out-alt"/>
        <p className="logout">Logout</p>
      </a>
      <Modal
        isOpen={visible}
        contentLabel="logout"
        className="logoutConfirmationBox"
        onRequestClose={close}
        overlayClassName="popupOverlay"
      >
        <h3>Confirm action</h3>
        <div className="separator"/>
        <p>Are you sure you want to logout?</p>
        <div className="options">
          <button className="confirmationBoxCancel" onClick={close}>Nevermind</button>
          <button className="confirmationBoxContinue" onClick={confirm}>Yes</button>
        </div>
      </Modal>
    </>
  );
};

const mapStateToProps = () => ({});

const mapActionsToProps = {
  logout
};

export default connect(mapStateToProps, mapActionsToProps)(LogoutBtn);
