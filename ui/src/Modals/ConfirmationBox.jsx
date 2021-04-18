import { useCallback } from "react";

import ModalBox from "./Index";

import "./ConfirmationBox.scss";

const ConfirmationBox = (props) => {
  const { action } = props;

  const confirmAction = useCallback(close => {
    close();
    action();
  }, [action]);

  return (
    <ModalBox activatingComponent={props.children}>
      {closeModal => (
        <div className="modalConfirmation">
          <h3>{props.title}</h3>
          <div className="separator"/>
          <p>{props.msg}</p>
          <div className="options">
            <button className="cancelBtn" onClick={closeModal}>
              {props.cancelText}
            </button>
            <button onClick={() => confirmAction(closeModal)}>
              {props.confirmText}
            </button>
          </div>
        </div>
      )}
    </ModalBox>
  )
};

export default ConfirmationBox;
