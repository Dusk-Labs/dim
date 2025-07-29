import { useCallback, useEffect, useState } from "react";
import { Link, useHistory } from "react-router-dom";
import { skipToken } from "@reduxjs/toolkit/query/react";
import Modal from "react-modal";

import { useGetMediaFilesQuery } from "../../api/v1/media";

import FileVideoIcon from "../../assets/Icons/FileVideo";
import { SelectMediaFileContext } from "./Context";
import Button from "../../Components/Misc/Button";

import "./Index.scss";

const SelectMediaFile = (props) => {
  const history = useHistory();

  /*
    prevents data from changing if e.g. banner in the
    background switches whilst user is still selecting
  */
  const [title, setTitle] = useState();
  const [currentID, setCurrentID] = useState();
  const [clicked, setClicked] = useState(false);

  const [visible, setVisible] = useState(false);
  const [ready, setReady] = useState(false);

  const { data: mediaFiles } = useGetMediaFilesQuery(
    currentID ? currentID : skipToken
  );

  useEffect(() => {
    if (currentID && visible) return;

    setTitle(props.title);
    setCurrentID(props.mediaID);
  }, [currentID, props.mediaID, props.title, visible]);

  // prevent scrolling behind Modal
  useEffect(() => {
    visible && ready
      ? (document.body.style.overflow = "hidden")
      : (document.body.style.overflow = "unset");
  }, [ready, visible]);

  const close = useCallback(() => {
    setVisible(false);
    setReady(false);
    setCurrentID();
    setTitle();
  }, []);

  const open = useCallback(() => {
    setVisible(true);
  }, []);

  useEffect(() => {
    if (!clicked || !currentID || !mediaFiles) return;

    if (mediaFiles.length === 1) {
      setClicked(false);
      if (
        history.location.state?.from &&
        history.location.state.from.startsWith("/play")
      ) {
        history.replace(`/play/${mediaFiles[0].id}`, {
          from: history.location.pathname,
        });
      } else {
        history.push(`/play/${mediaFiles[0].id}`, {
          from: history.location.pathname,
        });
      }
    } else {
      setClicked(false);
      open();
    }
  }, [clicked, currentID, history, mediaFiles, open]);

  const initialValue = {
    open,
    close,
    currentID,
    setClicked,
  };

  return (
    <SelectMediaFileContext.Provider value={initialValue}>
      <div id="modalSelectMediaFile">
        {props.children}
        <Modal
          isOpen={visible}
          className="modalBox"
          id="modalSelectMediaFile"
          onRequestClose={close}
          overlayClassName="popupOverlay"
        >
          {mediaFiles && mediaFiles.length === 0 && (
            <div className="modalSelectMediaFile">
              <div className="header">
                <h3>File selector</h3>
                <p className="desc">No files found for '{title}'</p>
              </div>
              <div className="separator" />
              <div className="err">
                <p>Empty</p>
              </div>
              <div className="options">
                <Button onClick={close}>Nevermind</Button>
              </div>
            </div>
          )}
          {mediaFiles && mediaFiles.length > 0 && (
            <div className="modalSelectMediaFile">
              <div className="header">
                <h3>Multiple files found</h3>
                <p className="desc">Choose a file to play for '{title}'</p>
              </div>
              <div className="separator" />
              <div className="fileVersionsWrapper">
                <div className="fileVersions">
                  {mediaFiles &&
                    mediaFiles.map((file, i) => (
                      <Link
                        to={`/play/${file.id}`}
                        className="fileVersion"
                        key={i}
                      >
                        <FileVideoIcon />
                        <p>{file.target_file.split(/\/|\\/g).pop()}</p>
                      </Link>
                    ))}
                </div>
              </div>
              <div className="options">
                <Button onClick={close}>Nevermind</Button>
              </div>
            </div>
          )}
        </Modal>
      </div>
    </SelectMediaFileContext.Provider>
  );
};

export default SelectMediaFile;
