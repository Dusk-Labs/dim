import { useCallback, useRef, useState } from "react";
import Modal from "react-modal";
import { useDispatch } from "react-redux";
import { newMediaFromMagnet } from "../../actions/library.js";

import ModalBox from "../Index.jsx";

import "./Index.scss";

Modal.setAppElement("body");

function NewMedia(props) {
  const linkInput = useRef(null);
  const [link, setLink] = useState("");
  const dispatch = useDispatch();

  const libId = props.libId;

  const add = useCallback(async (closeModal) => {
    if (!link) return;

    await dispatch(newMediaFromMagnet({link: link, lib: parseInt(libId)}));

    setLink("");
    closeModal();
  }, [link, dispatch, libId]);

  return (
    <ModalBox id="modalNewMedia" activatingComponent={props.children}>
      {(closeModal => (
        <div className="modalNewMedia">
          <div className="fields">
            <div className="field">
              <input
                ref={linkInput}
                onChange={e => setLink(e.target.value)}
                onKeyPress={(target) => target.key === "Enter" ? add(closeModal) : null}
                type="text"
                placeholder="Enter your magnet link here."
                value={link}
              />
            </div>
          </div>
        </div>
      ))}
    </ModalBox>
  );
}

export default NewMedia;
