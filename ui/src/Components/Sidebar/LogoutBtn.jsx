import React, { useState, useCallback, useEffect } from "react";
import { useHistory } from "react-router-dom";
import { connect } from "react-redux";
import Modal from "react-modal";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import ConfirmationBox from "../../Modals/ConfirmationBox.jsx";

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
    setVisible(false);
  }, []);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  const logout = useCallback(() => {
    props.logout();
    history.push("/login");
  }, []);

  return (
    <ConfirmationBox
      contentLabel="removeLib"
      action={logout}
      msg="Are you sure you want to logout?"
    >
      <a className="item logout" onClick={open}>
        <FontAwesomeIcon icon="sign-out-alt"/>
        <p className="logout">Logout</p>
      </a>
    </ConfirmationBox>
  );
};

const mapStateToProps = () => ({});

const mapActionsToProps = {
  logout
};

export default connect(mapStateToProps, mapActionsToProps)(LogoutBtn);
