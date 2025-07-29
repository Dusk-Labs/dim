import { cloneElement, useCallback, useEffect, useState } from "react";
import Modal from "react-modal";
import { useDispatch } from "react-redux";

import { newLibrary } from "../../actions/library.js";
import MediaTypeSelection from "./MediaTypeSelection";
import DirSelection from "./DirSelection";
import Field from "../../Pages/Auth/Field";
import Button from "../../Components/Misc/Button";

import "./Index.scss";

Modal.setAppElement("body");

function NewLibraryModal(props) {
  const dispatch = useDispatch();
  const [visible, setVisible] = useState(false);

  const [current, setCurrent] = useState("");
  const [name, setName] = useState("");
  const [nameErr, setNameErr] = useState("");
  const [mediaType, setMediaType] = useState("movie");
  const [selectedFolders, setSelectedFolders] = useState([]);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? (document.body.style.overflow = "hidden")
      : (document.body.style.overflow = "unset");
  }, [visible]);

  const clear = useCallback(() => {
    setName("");
    setCurrent("");
    setSelectedFolders([]);
    setMediaType("movie");
  }, []);

  const close = useCallback(() => {
    setVisible(false);
    clear();

    if (props.cleanUp) {
      props.cleanUp();
    }
  }, [clear, props]);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  useEffect(() => {
    if (!name) return;

    const movieRegex = new RegExp("movie|film", "gi");
    const tvShowRegex = new RegExp("tv|show|anime", "gi");
    const matchesMovie = movieRegex.test(name);
    const matchesTvOrShows = tvShowRegex.test(name);

    // TODO: set to 'mixed' when available.
    if (matchesMovie && matchesTvOrShows) {
      return;
    }

    if (matchesMovie) {
      setMediaType("movie");
    }

    if (matchesTvOrShows) {
      setMediaType("tv");
    }
  }, [name]);

  const add = useCallback(async () => {
    if (!name) {
      setNameErr("Label your library");
    }

    if (name && selectedFolders.length > 0) {
      const data = {
        name,
        locations: selectedFolders,
        media_type: mediaType,
      };

      dispatch(newLibrary(data));

      close();
    }
  }, [close, dispatch, mediaType, name, selectedFolders]);

  return (
    <div className="modalBoxContainer">
      {cloneElement(props.children, { onClick: () => open() })}
      <Modal
        isOpen={visible}
        className="modalBox"
        id="modalNewLibrary"
        onRequestClose={close}
        overlayClassName="popupOverlay"
      >
        <div className="modalNewLibrary">
          <div className="heading">
            <h3>Create a new library</h3>
            <div className="separator" />
          </div>
          <div className="fields">
            <Field
              name="Name"
              placeholder="Untitled"
              data={[name, setName]}
              error={[nameErr, setNameErr]}
            />
          </div>
          <MediaTypeSelection
            mediaType={mediaType}
            setMediaType={setMediaType}
          />
          <DirSelection
            current={current}
            setCurrent={setCurrent}
            selectedFolders={selectedFolders}
            setSelectedFolders={setSelectedFolders}
          />
          <div className="options">
            <Button type="secondary" onClick={close}>
              Nevermind
            </Button>
            <Button
              disabled={!name || selectedFolders.length === 0}
              onClick={add}
            >
              Add library
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}

export default NewLibraryModal;
