import { useCallback, useEffect, useRef, useState } from "react";
import Modal from "react-modal";
import { useDispatch } from "react-redux";

import { newLibrary } from "../../actions/library.js";
import MediaTypeSelection from "./MediaTypeSelection.jsx";
import DirSelection from "./DirSelection.jsx";
import ModalBox from "../Index.jsx";

import "./Index.scss";

Modal.setAppElement("body");

function NewLibraryModal(props) {
  const dispatch = useDispatch();

  const nameInput = useRef(null);

  const [current, setCurrent] = useState("");
  const [name, setName] = useState("Vol. " + "ABCDEFGHIJKLMNOPQRSTUVWXYZ".charAt(props.libCount % 26));
  const [mediaType, setMediaType] = useState("movie");
  const [libCount, setLibcount] = useState(props.libCount || 0);

  useEffect(() => {
    if (nameInput.current) {
      nameInput.current.style.border = "solid 2px transparent";
    }
  }, [name]);

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
      nameInput.current.style.border = "solid 2px #ff6961";
    }

    if (name) {
      const data = {
        name,
        location: current,
        media_type: mediaType
      };

      await dispatch(newLibrary(data));

      setName("Vol. " + "ABCDEFGHIJKLMNOPQRSTUVWXYZ".charAt(libCount + 1 % 26));
      setCurrent("");
      setMediaType("movie");
      setLibcount(libCount + 1);
      closeModal();
    }
  }, [current, dispatch, mediaType, name, libCount]);

  return (
    <ModalBox id="modalNewLibrary" activatingComponent={props.children}>
      {closeModal => (
        <div className="modalNewLibrary">
          <div className="heading">
            <h2>Add a new library</h2>
            <div className="separator"/>
          </div>
          <div className="fields">
            <div className="field">
              <h3>Name</h3>
              <input
                ref={nameInput}
                onChange={e => setName(e.target.value)}
                type="text"
                placeholder="Untitled"
                value={name}
              />
            </div>
          </div>
          <MediaTypeSelection
            mediaType={mediaType}
            setMediaType={setMediaType}
          />
          <DirSelection
            current={current}
            setCurrent={setCurrent}
          />
          <div className="options">
            <button onClick={closeModal}>Nevermind</button>
            <button onClick={() => add(closeModal)}>Add library</button>
          </div>
        </div>
      )}
    </ModalBox>
  );
}

export default NewLibraryModal;
