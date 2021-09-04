import { useCallback, useContext } from "react";

import { SelectUnmatchedContext } from "./Context";
import CheckIcon from "../../../assets/Icons/Check";

import "./FileControls.scss";
import { LibraryContext } from "../Context";

const SelectUnmatchedMediaFileControls = () => {
  const { unmatched } = useContext(LibraryContext);
  const { selectedFiles, currentFolder, setSelectedFiles } = useContext(SelectUnmatchedContext);

  const selectAll = useCallback(() => {
    const selected = Object.values(selectedFiles).filter(file => file.parent === currentFolder);

    if (selected.length === unmatched.items[currentFolder].length) {
      const newSelectedFiles = {};

      for (const id in selectedFiles) {
        if (selectedFiles[id].parent === currentFolder) continue;
        newSelectedFiles[id] = selectedFiles[id];
      }

      setSelectedFiles(newSelectedFiles);

      return;
    }

    for (const file of unmatched.items[currentFolder]) {
      setSelectedFiles(state => ({
        ...state,
        [file.id]: {
          id: file.id,
          parent: currentFolder
        }
      }));
    }
  }, [currentFolder, selectedFiles, setSelectedFiles, unmatched.items]);

  const selected = Object.values(selectedFiles).filter(file => file.parent === currentFolder);
  const allFilesInCurrentFolderSelected = selected.length === unmatched.items[currentFolder]?.length;

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
