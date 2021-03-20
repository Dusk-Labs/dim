import { useCallback, useEffect, useState } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { fetchDirectories } from "../../actions/fileBrowser.js";

import "./DirSelection.scss";

function DirSelection(props) {
  const [cache, setCache] = useState(false);

  const { current, fetchDirectories, setCurrent, auth, fileBrowser } = props;

  const select = useCallback(path => {
    if (path in fileBrowser.cache) {
      setCurrent(path);
      setCache(true);

      return;
    }

    fetchDirectories(auth.token, path.replace("C:\\", ""));
    setCurrent(path);

    setCache(false);
  }, [auth.token, fetchDirectories, fileBrowser.cache, setCurrent]);

  useEffect(() => {
    const path = "";

    fetchDirectories(auth.token, path.replace("C:\\", ""));
    setCurrent(path);
    setCache(false);
  }, [auth.token, fetchDirectories, setCurrent]);

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
    if (props.fileBrowser.fetched && props.fileBrowser.error) {
      dirs = (
        <div className="vertical-err">
          <p>Cannot load data</p>
        </div>
      );
    }

    // FETCH_DIRECTORIES_OK
    if (props.fileBrowser.fetched && !props.fileBrowser.error) {
      const { items } = props.fileBrowser;

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
    const items = props.fileBrowser.cache[props.current];

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

const mapStateToProps = (state) => ({
  auth: state.auth,
  fileBrowser: state.fileBrowser
});

const mapActionsToProps = {
  fetchDirectories
};

export default connect(mapStateToProps, mapActionsToProps)(DirSelection);
