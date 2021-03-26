import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { cloneElement, useCallback, useEffect, useState } from "react";
import Modal from "react-modal";
import { Link } from "react-router-dom";

import "./SelectMediaVersion.scss";

const SelectMediaVersion = (props) => {
  const [visible, setVisible] = useState(false);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? document.body.style.overflow = "hidden"
      : document.body.style.overflow = "unset";
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
        className="selectMediaVersion"
        onRequestClose={close}
        overlayClassName="popupOverlay"
      >
        <h3>Select file version</h3>
        <div className="separator"/>
        <div className="fileVersionsWrapper">
          <div className="fileVersions">
            {props.versions.map((version, i) => (
              <Link to={`/play/${version.id}`} className="fileVersion" key={i}>
                <FontAwesomeIcon icon="file-video"/>
                <p>{version.display_name}</p>
              </Link>
            ))}
          </div>
        </div>
        <div className="options">
          <button className="selectMediaVersionCancel" onClick={close}>Nevermind</button>
        </div>
      </Modal>
    </div>
  )
};

export default SelectMediaVersion;
