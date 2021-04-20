import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { fetchDirectories } from "../../actions/fileBrowser.js";

import "./DirSelection.scss";

function DirSelection(props) {
  const dispatch = useDispatch();
  const fileBrowser = useSelector(store => store.fileBrowser);

  const [cache, setCache] = useState(false);

  const { current, setCurrent } = props;

  const select = useCallback(path => {
    if (path in fileBrowser.cache) {
      setCurrent(path);
      setCache(true);

      return;
    }

    dispatch(fetchDirectories(path.replace("C:\\", "")));
    setCurrent(path);

    setCache(false);
  }, [dispatch, fileBrowser.cache, setCurrent]);

  useEffect(() => {
    const path = "";

    dispatch(fetchDirectories(path.replace("C:\\", "")));
    setCurrent(path);
    setCache(false);
  }, [dispatch, setCurrent]);

  const goBack = useCallback(() => {
    let slash = "/";

    if (current.includes("\\")) {
      slash = "\\";
    }

    if (current.length === 0) return;

    const path = current.split(slash);

    path.pop();

    select(path.join(slash));
  }, [current, select])

  let dirs;

  if (!cache) {
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
          return (
            <div key={i} onClick={() => select(dir)} className="dir">
              <FontAwesomeIcon icon="folder"/>
              <p>{dir.replace(props.current, "").replace("C:\\", "").replace("/", "").replace("\\", "")}</p>
            </div>
          )
        });
      }
    }
  } else {
    const items = fileBrowser.cache[props.current];

    if (items.length === 0) {
      dirs = (
        <div className="vertical-err">
          <FontAwesomeIcon icon="times-circle"/>
          <p>NO FOLDERS</p>
        </div>
      );
    } else {
      dirs = items.map((dir, i) => (
        <div key={i} onClick={() => select(dir)} className="dir">
          <FontAwesomeIcon icon="folder"/>
          <p>{dir.replace(props.current, "").replace("C:\\", "").replace("/", "").replace("\\", "")}</p>
        </div>
      ));
    }
  }

  return (
    <div className="dirSelection">
      <h3>Select folder</h3>
      <div className="dirs-wrapper">
        <div className="dirs">
          {dirs}
        </div>
      </div>
      <div className="controls">
        <button onClick={goBack}>
          <FontAwesomeIcon icon="arrow-left"/>
        </button>
        <h4>Selected: <span>{props.current}</span></h4>
      </div>
    </div>
  )
};

export default DirSelection;
