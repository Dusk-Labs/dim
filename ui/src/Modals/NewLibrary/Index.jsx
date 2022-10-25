import { cloneElement, useCallback, useEffect, useState } from "react";
import Modal from "react-modal";
import { useDispatch, useSelector } from "react-redux";

import { newLibrary } from "../../actions/library.js";
import MediaTypeSelection from "./MediaTypeSelection";
import DirSelection from "./DirSelection";
import Field from "../../Pages/Auth/Field";
import Button from "../../Components/Misc/Button";

import "./Index.scss";

Modal.setAppElement("body");

function NewLibraryModal(props) {
  const dispatch = useDispatch();
  const libraryNames = useSelector((store) => store.library.fetch_libraries);
  const allNames = libraryNames.items;
  const [visible, setVisible] = useState(false);

  const [current, setCurrent] = useState("");
  const [name, setName] = useState("");
  const [nameErr, setNameErr] = useState("");
  const [mediaType, setMediaType] = useState("");
  const [selectedFolders, setSelectedFolders] = useState([]);
  const [placeHolderName, setPlaceHolderName] = useState("Untitled");
  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? (document.body.style.overflow = "hidden")
      : (document.body.style.overflow = "unset");
  }, [visible]);

  const clear = useCallback(() => {
    setPlaceHolderName("Untitled");
    setName("");
    setCurrent("");
    setSelectedFolders([]);
    setMediaType("");
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
    if (mediaType === "tv") {
      setName(placeHolderName);
    }
    if (mediaType === "movie") {
      setName(placeHolderName);
    }

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
  }, [close, dispatch, mediaType, name, selectedFolders, placeHolderName]);

  const checkIfName = useCallback(() => {
    let label;

    if (mediaType === "tv") {
      label = "TV shows library";
    } else {
      label = "Movie library";
    }

    const repeatedNames = allNames.filter((i) => i.name.includes(label));

    setPlaceHolderName(label);

    if (repeatedNames.length && allNames.length > 0) {
      for (var i = 0; i < repeatedNames.length; i++) {
        var libName = repeatedNames[i].name;

        if (libName === repeatedNames[repeatedNames.length - 1].name) {
          setPlaceHolderName(`${label} ${i + 1}`);

          break;
        }
      }
    }
  }, [allNames, mediaType, setPlaceHolderName]);

  useEffect(() => {
    const checkName = libraryNames.items.filter(
      (value) => value.name === placeHolderName
    );

    if (mediaType === "tv" && checkName.length >= 0) {
      checkIfName();
    }
    if (mediaType === "movie" && checkName.length >= 0) {
      checkIfName();
    }
  }, [mediaType, setMediaType, checkIfName, placeHolderName, libraryNames]);

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
              placeholder={placeHolderName}
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
            <Button disabled={selectedFolders.length === 0} onClick={add}>
              Add library
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}

export default NewLibraryModal;
