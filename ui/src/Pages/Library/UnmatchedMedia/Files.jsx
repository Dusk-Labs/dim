import { useCallback, useContext } from "react";

import FileControls from "./FileControls";
import CheckIcon from "../../../assets/Icons/Check";
import { SelectUnmatchedContext } from "./Context";
import { LibraryContext } from "../Context";

import "./Files.scss";

const SelectUnmatchedMediaFiles = () => {
  const { unmatched } = useContext(LibraryContext);
  const { selectedFiles, currentFolder, setSelectedFiles } = useContext(SelectUnmatchedContext);

  const selectFile = useCallback((file) => {
    if (selectedFiles[file.id]) {
      const newSelectedFiles = {};

      for (const id in selectedFiles) {
        if (parseInt(id) === file.id) continue;
        newSelectedFiles[id] = selectedFiles[id];
      }

      setSelectedFiles(newSelectedFiles);

      return;
    }

    setSelectedFiles(state => ({
      ...state,
      [file.id]: {
        id: file.id,
        name: file.target_file.split(/\/|\\/g).pop(),
        parent: currentFolder
      }
    }));
  }, [currentFolder, selectedFiles, setSelectedFiles]);

  return (
    <div className="selectUnmatchedMediaFiles">
      <FileControls/>
      <div className="files">
        {unmatched.items[currentFolder].map((file, i) => (
          <div
            key={i}
            className={`file selected-${!!selectedFiles[file.id]}`}
            onClick={() => selectFile(file)}
          >
            <div className="selectBox">
              <CheckIcon/>
            </div>
            <p>{file.target_file.split(/\/|\\/g).pop()}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default SelectUnmatchedMediaFiles;
