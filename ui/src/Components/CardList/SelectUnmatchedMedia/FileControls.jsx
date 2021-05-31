import { useCallback, useContext } from "react";

import { SelectUnmatchedContext } from "./Context";
import CheckIcon from "../../../assets/Icons/Check";

import "./FileControls.scss";

const SelectUnmatchedMediaFileControls = () => {
  const { items, selectedFiles, currentFolder, setSelectedFiles } = useContext(SelectUnmatchedContext);

  const selectAll = useCallback(() => {
    const selected = Object.values(selectedFiles).filter(file => file.parent === currentFolder);

    if (selected.length === items[currentFolder].length) {
      const newSelectedFiles = {};

      for (const name in selectedFiles) {
        if (selectedFiles[name].parent === currentFolder) continue;
        newSelectedFiles[name] = selectedFiles[name];
      }

      setSelectedFiles(newSelectedFiles);

      return;
    }

    for (const file of items[currentFolder]) {
      setSelectedFiles(state => ({
        ...state,
        [file.name]: {
          id: file.id,
          parent: currentFolder
        }
      }));
    }
  }, [currentFolder, items, selectedFiles, setSelectedFiles]);

  const selected = Object.values(selectedFiles).filter(file => file.parent === currentFolder);
  const allFilesInCurrentFolderSelected = selected.length === items[currentFolder]?.length;

  return (
    <div className="selectUnmatchedMediaFileControls">
      <button className={`selectAll active-${allFilesInCurrentFolderSelected}`} onClick={selectAll}>
        <div className="selectBox">
          <CheckIcon/>
        </div>
        {allFilesInCurrentFolderSelected
          ? "Clear selection"
          : "Select all"
        }
      </button>
    </div>
  );
};

export default SelectUnmatchedMediaFileControls;
