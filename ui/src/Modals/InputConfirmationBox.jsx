import { useCallback } from "react";

import ModalBox from "./Index";
import Field from "../Pages/Auth/Field";

import "./InputConfirmationBox.scss";

const ConfirmationBox = (props) => {
  const { action } = props;

  const confirmAction = useCallback(async close => {
    const valid = await action();

    if (valid) {
      close();
    }
  }, [action]);

  return (
    <ModalBox activatingComponent={props.children}>
      {closeModal => (
        <div className="modalConfirmation">
          <h3>{props.title}</h3>
          <div className="separator"/>
          <p className="desc">{props.msg}</p>
          <Field
            name={props.label}
            icon={props.icon}
            data={[props.data, props.setData]}
            error={[props.err, props.setErr]}
            type={props.type}
          />
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
  );
};

export default ConfirmationBox;
