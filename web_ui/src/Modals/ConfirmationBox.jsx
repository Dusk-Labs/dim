import React, { cloneElement, useCallback, useEffect, useState } from "react";
import Modal from "react-modal";

import "./ConfirmationBox.scss";

const ConfirmationBox = (props) => {
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

  return (
    <div className="confirmationAction">
      {cloneElement(props.children, { onClick: () => open() })}
      <Modal
        isOpen={visible}
        contentLabel={props.contentLabel}
        className="confirmationBox"
        onRequestClose={close}
        overlayClassName="popupOverlay"
      >
        <h3>Confirm action</h3>
        <div className="separator"/>
        <p>{props.msg}</p>
        <div className="options">
          <button className="confirmationBoxCancel" onClick={close}>Nevermind</button>
          <button className="confirmationBoxContinue" onClick={props.action}>Yes</button>
        </div>
      </Modal>
    </div>
  )
};

export default ConfirmationBox;
