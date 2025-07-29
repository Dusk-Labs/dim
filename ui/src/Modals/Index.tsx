import React, { cloneElement, useCallback, useEffect, useState } from "react";
import Modal from "react-modal";

import "./Index.scss";

interface Props {
  activatingComponent?: React.ReactElement;
  id?: string;
  children: (close: () => void) => React.ReactNode;
}

/*
  This contains the core logic for a modal e.g. open/close etc
  This is to only be used by components that intend to be rendered in a modal
*/
const ModalBox = (props: Props) => {
  const [visible, setVisible] = useState(!props.activatingComponent);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? (document.body.style.overflow = "hidden")
      : (document.body.style.overflow = "unset");
  }, [visible]);

  const close = useCallback(() => {
    setVisible(false);
  }, []);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  return (
    <div className="modalBoxContainer">
      {props.activatingComponent &&
        cloneElement(props.activatingComponent, { onClick: () => open() })}
      <Modal
        isOpen={visible}
        className="modalBox"
        id={props.id}
        onRequestClose={close}
        overlayClassName="popupOverlay"
      >
        {props.children(close)}
      </Modal>
    </div>
  );
};

export default ModalBox;
