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
          <h3>Confirm action</h3>
          <div className="separator"/>
          <p>{props.msg}</p>
          <div className="options">
            <button className="cancelBtn" onClick={closeModal}>Nevermind</button>
            <button onClick={() => confirmAction(closeModal)}>Yes</button>
          </div>
        </div>
      )}
    </ModalBox>
  )
};

export default ConfirmationBox;
