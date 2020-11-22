import React, { useCallback, useEffect, useState } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { fetchDirectories } from "../../../actions/fileBrowser.js";

import "./DirSelection.scss";

function DirSelection(props) {
  const [cache, setCache] = useState(false);
  const [slash, setSlash] = useState("/");

  const select = useCallback(path => {
    if (path === props.current) return;

    if (path in props.fileBrowser.cache) {
      props.setCurrent(path.replace("/", slash));
      setCache(true);

      return;
    }

    props.fetchDirectories(props.auth.token, path);
    props.setCurrent(path.replace("/", slash));

    setCache(false);
  }, [props.current, props.fileBrowser]);

  useEffect(() => {
    // use slash other way if windows
    if (navigator.appVersion.indexOf("Win") !== -1) {
      setSlash("\\");
      select("\\");
    } else {
      select("/");
    }
  }, [])

  const goBack = useCallback(() => {
    if (props.current === slash) return;
    const path = props.current.split(slash);

    path.pop();

    select(path.join(slash) || slash);
  }, [slash, props.current])

  let dirs;

  if (!cache) {
    // FETCH_DIRECTORIES_ERR
    if (props.fileBrowser.fetched && props.fileBrowser.error) {
      dirs = (
        <div className="vertical-err">
          <FontAwesomeIcon icon="times-circle"/>
          <p>Cannot load data</p>
        </div>
      );
    }

    // FETCH_DIRECTORIES_OK
    if (props.fileBrowser.fetched && !props.fileBrowser.error) {
      const { items } = props.fileBrowser;

      console.log("items", items)

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
              <p>{dir.replace("/", slash).replace(props.current, "").replace(slash, "")}</p>
            </div>
          )
        });
      }
    }
  } else {
    const items = props.fileBrowser.cache[props.current];

    console.log("PROPS", props.fileBrowser.cache, "CURRENT", props.current)

    console.log("items cache", items)

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
          <p>{dir.replace("/", slash).replace(props.current, "").replace(slash, "")}</p>
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