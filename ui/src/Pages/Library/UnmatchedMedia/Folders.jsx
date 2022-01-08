import { useCallback, useContext } from "react";

import FolderIcon from "../../../assets/Icons/Folder";
import { LibraryContext } from "../Context";
import { SelectUnmatchedContext } from "./Context";

import "./Folders.scss";

const SelectUnmatchedMediaFolders = () => {
  const { unmatched } = useContext(LibraryContext);
  const { selectedFiles, currentFolder, setCurrentFolder } = useContext(
    SelectUnmatchedContext
  );

  const selectFolder = useCallback(
    (folder) => {
      if (currentFolder === folder) return;
      setCurrentFolder(folder);
    },
    [currentFolder, setCurrentFolder]
  );

  const folders = Object.keys(unmatched.items).map((folder, i) => {
    const files = Object.values(selectedFiles);
    const count = files.filter((file) => file.parent === folder).length;

    return (
      <div
        title={folder}
        key={i}
        className={`folder selected-${currentFolder === folder}`}
        onClick={() => selectFolder(folder)}
      >
        <div className="folderIcon">
          <FolderIcon />
          {count > 0 && <p>{count}</p>}
        </div>
        <p>{folder}</p>
      </div>
    );
  });

  return <div className="selectUnmatchedMediaFolders">{folders}</div>;
};

export default SelectUnmatchedMediaFolders;
