import { useCallback, useEffect, useState } from "react";
import Modal from "react-modal";
import { useDispatch } from "react-redux";

import { newLibrary } from "../../actions/library.js";
import MediaTypeSelection from "./MediaTypeSelection.jsx";
import DirSelection from "./DirSelection.jsx";
import ModalBox from "../Index.jsx";
import Field from "../../Pages/Auth/Field.jsx";
import Button from "../../Components/Misc/Button.jsx";

import "./Index.scss";

Modal.setAppElement("body");

function NewLibraryModal(props) {
  const dispatch = useDispatch();

  const [current, setCurrent] = useState("");
  const [name, setName] = useState("");
  const [nameErr, setNameErr] = useState("");
  const [mediaType, setMediaType] = useState("movie");
  const [selectedFolders, setSelectedFolders] = useState([]);

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

  const add = useCallback(async (closeModal) => {
    if (!name) {
      setNameErr("Label your library");
    }

    if (name && selectedFolders.length > 0) {
      const data = {
        name,
        locations: selectedFolders,
        media_type: mediaType
      };

      await dispatch(newLibrary(data));

      setName("");
      setCurrent("");
      setSelectedFolders([]);
      setMediaType("movie");
      closeModal();
    }
  }, [dispatch, mediaType, name, selectedFolders]);

  return (
    <ModalBox id="modalNewLibrary" activatingComponent={props.children}>
      {closeModal => (
        <div className="modalNewLibrary">
          <div className="heading">
            <h3>Create a new library</h3>
            <div className="separator"/>
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
            <Button
              type="secondary"
              onClick={closeModal}
            >Nevermind</Button>
            <Button
              disabled={!name || selectedFolders.length === 0}
              onClick={() => add(closeModal)}>
              Add library
            </Button>
          </div>
        </div>
      )}
    </ModalBox>
  );
}

export default NewLibraryModal;
