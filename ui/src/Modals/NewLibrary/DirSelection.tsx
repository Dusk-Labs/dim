import { useCallback, useState } from "react";

import { useGetDirectoriesQuery } from "../../api/v1/fileBrowser";
import FolderIcon from "../../assets/Icons/Folder";
import CheckIcon from "../../assets/Icons/Check";
import Button from "../../Components/Misc/Button";
import ChevronRight from "../../assets/Icons/ChevronRight";
import ChevronLeft from "../../assets/Icons/ChevronLeft";

import "./DirSelection.scss";

interface Props {
  current: string;
  setCurrent: React.Dispatch<React.SetStateAction<string>>;
  selectedFolders: string[];
  setSelectedFolders: React.Dispatch<React.SetStateAction<string[]>>;
}

function DirSelection(props: Props) {
  const { current, setCurrent, selectedFolders, setSelectedFolders } = props;
  const [forwardHistory, setForwardHistory] = useState<string[]>([]);

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
      const history = [...forwardHistory];

      // This truncates our forward history till we get to the item thats equivalent to our current path.
      while (history.length !== 0) {
        if (history.pop() === current) break;
      }

      setForwardHistory(history);
      setCurrent(path);
    },
    [current, setCurrent, forwardHistory, setForwardHistory]
  );

  const goBack = useCallback(() => {
    if (current.length === 0) return;

    const history = [...forwardHistory];
    const path = current.split("/");

    history.push(current);
    path.pop();
    setCurrent(path.join("/"));
    setForwardHistory(history);
  }, [current, setCurrent, forwardHistory, setForwardHistory]);

  const goForward = useCallback(() => {
    const history = [...forwardHistory];
    if (history.length === 0) return;

    setCurrent(history.pop()!);
    setForwardHistory(history);
  }, [setCurrent, forwardHistory, setForwardHistory]);

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
            <div className="selectBox" onClick={() => selectFolder(dir)}>
              <CheckIcon />
            </div>
            <div className="label" onClick={() => select(dir)}>
              <FolderIcon />
              <p>
                {dir.replace(current, "").replace("/", "")}
                <span className="selectedInsideCount">
                  {count
                    ? ` ${count} ${
                        count === 1 ? "folder" : "folders"
                      } selected inside`
                    : ""}
                </span>
              </p>
            </div>
            <div className="chevron-hint">
              <ChevronRight />
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
      <div className="controls">
        <Button
          onClick={goBack}
          disabled={current === ""}
          type="secondary contrast"
        >
          <ChevronLeft />
        </Button>
        <Button
          onClick={goForward}
          disabled={forwardHistory.length === 0}
          type="secondary contrast"
        >
          <ChevronRight />
        </Button>
        <div className="current-folder-label">
          <p className="current-label">Currently in:</p>
          <p className="current">{current?.length === 0 ? "/" : current}</p>
        </div>
      </div>
      <div className="dirs-wrapper">
        <div className="dirs">{dirs}</div>
      </div>
      <div className="header">
        <div className="folders-selected-cnt">
          <h4>Folders selected:</h4>
          <h4>{selectedFolders.length}</h4>
        </div>
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
    </div>
  );
}

export default DirSelection;
