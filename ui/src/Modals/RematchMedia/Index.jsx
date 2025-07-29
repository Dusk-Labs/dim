import { cloneElement, useCallback, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import Modal from "react-modal";
import { useDispatch, useSelector } from "react-redux";

import { addNotification } from "../../slices/notifications";
import MediaTypeSelection from "./MediaTypeSelection";
import Button from "../../Components/Misc/Button";

import Search from "./Search";
import { RematchContext } from "./Context";

import "./Index.scss";

Modal.setAppElement("body");

function RematchMediaModal(props) {
  const dispatch = useDispatch();

  const { id } = useParams();

  const { token } = useSelector((store) => ({
    token: store.auth.token,
  }));

  const [visible, setVisible] = useState(false);

  const [query, setQuery] = useState("");
  const [tmdbResults, setTmdbResults] = useState([]);
  const [mediaType, setMediaType] = useState("movie");
  const [tmdbID, setTmdbID] = useState();
  const [error, setError] = useState("");
  const [matching, setMatching] = useState(false);

  const clearData = useCallback(() => {
    setTmdbID();
    setTmdbResults([]);
    setError();
    setQuery();
    setMatching(false);
  }, []);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible
      ? (document.body.style.overflow = "hidden")
      : (document.body.style.overflow = "unset");
  }, [visible]);

  const close = useCallback(() => {
    setVisible(false);
    clearData();

    if (props.cleanUp) {
      props.cleanUp();
    }
  }, [props, clearData]);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  const rematch = useCallback(async () => {
    if (!tmdbID || !mediaType) return;

    setMatching(true);

    const config = {
      method: "PATCH",
      headers: {
        authorization: token,
      },
    };

    const req = await fetch(
      `/api/v1/media/${id}/match?external_id=${tmdbID}&media_type=${mediaType}`,
      config
    );

    if (req.status !== 200) {
      setError(req.statusText);
      return;
    }

    dispatch(
      addNotification({
        msg: `Sucessfuly matched ${id}.`,
      })
    );

    clearData();
    close();
  }, [
    clearData,
    dispatch,
    mediaType,
    id,
    setError,
    setMatching,
    tmdbID,
    token,
    close,
  ]);

  const initialValue = {
    mediaType,
    setMediaType,
    tmdbResults,
    setTmdbResults,
    query,
    setQuery,
    tmdbID,
    setTmdbID,
  };

  return (
    <RematchContext.Provider value={initialValue}>
      <div className="modalBoxContainer">
        {cloneElement(props.children, { onClick: () => open() })}
        <Modal
          isOpen={visible}
          className="modalBox"
          id="modalRematchMedia"
          onRequestClose={close}
          overlayClassName="popupOverlay"
        >
          <div className="modalRematchMedia">
            <div className="heading">
              <h3>Rematch</h3>
              <div className="separator" />
            </div>
            <MediaTypeSelection
              mediaType={mediaType}
              setMediaType={setMediaType}
            />
            <div className="search-section">
              <Search />
            </div>
            {error && <p className="err">Error: {error}</p>}
            <div className="options">
              <Button type="secondary" onClick={close}>
                Nevermind
              </Button>
              <Button onClick={rematch} disabled={matching || false}>
                Match
              </Button>
            </div>
          </div>
        </Modal>
      </div>
    </RematchContext.Provider>
  );
}

export default RematchMediaModal;
