import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { fetchDirectories } from "../../actions/fileBrowser.js";
import FolderIcon from "../../assets/Icons/Folder";
import ArrowLeftIcon from "../../assets/Icons/ArrowLeft";
import CheckIcon from "../../assets/Icons/Check";
import Button from "../../Components/Misc/Button";

import "./DirSelection.scss";

function DirSelection(props) {
  const dispatch = useDispatch();
  const fileBrowser = useSelector(store => store.fileBrowser);

  const { current, setCurrent, selectedFolders, setSelectedFolders } = props;

  const selectFolder = useCallback(path => {
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
      setSelectedFolders(state => [...state, path]);
    }
  }, [selectedFolders, setSelectedFolders]);

  const selectAllFolders = useCallback(() => {
    const unselectedFolders = fileBrowser.items.filter(item => (
      !selectedFolders.includes(item)
    ));

    setSelectedFolders(state => unselectedFolders.concat(state));
  }, [fileBrowser.items, selectedFolders, setSelectedFolders]);

  const clearSelection = useCallback(() => {
    setSelectedFolders([]);
  }, [setSelectedFolders]);

  const select = useCallback(path => {
    dispatch(fetchDirectories(path));
    setCurrent(path);
  }, [dispatch, setCurrent]);

  useEffect(() => {
    const path = "";

    dispatch(fetchDirectories(path));
    setCurrent(path);
  }, [dispatch, setCurrent]);

  const goBack = useCallback(() => {
    if (current.length === 0) return;

    const path = current.split("/");

    path.pop();
    select(path.join("/"));
  }, [current, select]);

  let dirs;

  // FETCH_DIRECTORIES_ERR
  if (fileBrowser.fetched && fileBrowser.error) {
    dirs = (
      <div className="vertical-err">
        <p>Cannot load data</p>
      </div>
    );
  }

  // FETCH_DIRECTORIES_OK
  if (fileBrowser.fetched && !fileBrowser.error) {
    const { items } = fileBrowser;

    if (items.length === 0) {
      dirs = (
        <div className="vertical-err">
          <p>Empty</p>
        </div>
      );
    } else {
      dirs = items.map((dir, i) => {
        const count = selectedFolders.filter(folder => (
          folder.includes(dir + "/") && folder !== dir
        )).length;

        return (
          <div
            key={i}
            className={`dir selected-${selectedFolders.includes(dir)}`}
          >
            <div className="label" onClick={() => select(dir)}>
              <FolderIcon/>
              <p>
                {dir.replace(props.current, "").replace("/", "")}
                <span className="selectedInsideCount">{count ? ` (${count} folders selected inside)` : ""}</span>
              </p>
            </div>
            <div className="selectBox" onClick={() => selectFolder(dir)}>
              <CheckIcon/>
            </div>
          </div>
        );
      });
    }
  }

  const allFoldersInCurrentSelected = fileBrowser.items.every(item => (
    selectedFolders.includes(item)
  ));

  return (
    <div className="dirSelection">
      <div className="header">
        <h4>Select folders containing media ({selectedFolders.length})</h4>
        <div className="actions">
          <Button
            disabled={selectedFolders.length <= 0}
            type="secondary"
            onClick={clearSelection}
          >Clear all</Button>
          <Button
            disabled={allFoldersInCurrentSelected}
            type="secondary"
            onClick={selectAllFolders}
          >Select all</Button>
        </div>
      </div>
      <div className="dirs-wrapper">
        <div className="dirs">
          {dirs}
        </div>
      </div>
      <div className="controls">
        <Button onClick={goBack} disabled={props.current === ""}>
          <ArrowLeftIcon/>
        </Button>
        <p className="current">{props.current}</p>
      </div>
    </div>
  );
}

export default DirSelection;
