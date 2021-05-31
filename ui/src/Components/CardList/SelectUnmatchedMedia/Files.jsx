import { useCallback, useContext } from "react";

import FileControls from "./FileControls";
import CheckIcon from "../../../assets/Icons/Check";
import { SelectUnmatchedContext } from "./Context";

import "./Files.scss";

const SelectUnmatchedMediaFiles = () => {
  const { items, selectedFiles, currentFolder, setSelectedFiles } = useContext(SelectUnmatchedContext);

  const selectFile = useCallback((file) => {
    if (selectedFiles[file.name]) {
      const newSelectedFiles = {};

      for (const name in selectedFiles) {
        if (name === file.name) continue;
        newSelectedFiles[name] = selectedFiles[name];
      }

      setSelectedFiles(newSelectedFiles);

      return;
    }

    setSelectedFiles(state => ({
      ...state,
      [file.name]: {
        id: file.id,
        parent: currentFolder
      }
    }));
  }, [currentFolder, selectedFiles, setSelectedFiles]);

  return (
    <div className="selectUnmatchedMediaFiles">
      <FileControls/>
      <div className="files">
        {items[currentFolder].map((file, i) => (
          <div
            key={i}
            className={`file selected-${!!selectedFiles[file.name]}`}
            onClick={() => selectFile(file)}
          >
            <div className="selectBox">
              <CheckIcon/>
            </div>
            <p>{file.name}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default SelectUnmatchedMediaFiles;
