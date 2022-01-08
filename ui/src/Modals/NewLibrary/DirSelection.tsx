import { useCallback } from "react";

import { useGetDirectoriesQuery } from "../../api/v1/fileBrowser";
import FolderIcon from "../../assets/Icons/Folder";
import ArrowLeftIcon from "../../assets/Icons/ArrowLeft";
import CheckIcon from "../../assets/Icons/Check";
import Button from "../../Components/Misc/Button";

import "./DirSelection.scss";

interface Props {
  current: string;
  setCurrent: React.Dispatch<React.SetStateAction<string>>;
  selectedFolders: string[];
  setSelectedFolders: React.Dispatch<React.SetStateAction<string[]>>;
}

function DirSelection(props: Props) {
  const { current, setCurrent, selectedFolders, setSelectedFolders } = props;

  const { data: items, error, isFetching } = useGetDirectoriesQuery(current);

  const selectFolder = useCallback(
    (path) => {
      const alreadySelected = selectedFolders.includes(path);

      if (alreadySelected) {
        const newSelectedFolders = [];

        for (const name of selectedFolders) {
          if (name === path) continue;
          newSelectedFolders.push(name);
        }

        setSelectedFolders(newSelectedFolders);

        return;
      }

      if (!alreadySelected) {
        setSelectedFolders((state) => [...state, path]);
      }
    },
    [selectedFolders, setSelectedFolders]
  );

  const selectAllFolders = useCallback(() => {
    if (!items) {
      return;
    }

    const unselectedFolders = items.filter(
      (item) => !selectedFolders.includes(item)
    );

    setSelectedFolders((state) => unselectedFolders.concat(state));
  }, [items, selectedFolders, setSelectedFolders]);

  const clearSelection = useCallback(() => {
    setSelectedFolders([]);
  }, [setSelectedFolders]);

  const select = useCallback(
    (path) => {
      setCurrent(path);
    },
    [setCurrent]
  );

  const goBack = useCallback(() => {
    if (current.length === 0) return;

    const path = current.split("/");

    path.pop();
    select(path.join("/"));
  }, [current, select]);

  let dirs;

  if (isFetching || error) {
    dirs = (
      <div className="vertical-err">
        <p>Cannot load data</p>
      </div>
    );
  } else if (items) {
    if (items.length === 0) {
      dirs = (
        <div className="vertical-err">
          <p>Empty</p>
        </div>
      );
    } else {
      dirs = items.map((dir, i) => {
        const count = selectedFolders.filter(
          (folder) => folder.includes(dir + "/") && folder !== dir
        ).length;

        return (
          <div
            key={i}
            className={`dir selected-${selectedFolders.includes(dir)}`}
          >
            <div className="label" onClick={() => select(dir)}>
              <FolderIcon />
              <p>
                {dir.replace(current, "").replace("/", "")}
                <span className="selectedInsideCount">
                  {count ? ` (${count} folders selected inside)` : ""}
                </span>
              </p>
            </div>
            <div className="selectBox" onClick={() => selectFolder(dir)}>
              <CheckIcon />
            </div>
          </div>
        );
      });
    }
  }

  const allFoldersInCurrentSelected =
    items && items.every((item) => selectedFolders.includes(item));

  return (
    <div className="dirSelection">
      <div className="header">
        <h4>Select folders containing media ({selectedFolders.length})</h4>
        <div className="actions">
          <Button
            disabled={selectedFolders.length <= 0}
            type="secondary"
            onClick={clearSelection}
          >
            Clear all
          </Button>
          <Button
            disabled={allFoldersInCurrentSelected}
            type="secondary"
            onClick={selectAllFolders}
          >
            Select all
          </Button>
        </div>
      </div>
      <div className="dirs-wrapper">
        <div className="dirs">{dirs}</div>
      </div>
      <div className="controls">
        <Button onClick={goBack} disabled={current === ""}>
          <ArrowLeftIcon />
        </Button>
        <p className="current">{current}</p>
      </div>
    </div>
  );
}

export default DirSelection;
