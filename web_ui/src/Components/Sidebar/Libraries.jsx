import React, { useEffect, useState } from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";

import { fetchLibraries, delLibrary, handleWsNewLibrary, handleWsDelLibrary } from "../../actions/library.js";
import SidebarIcon from "./Icon.jsx";
import NewLibraryModal from "./NewLibraryModal.jsx";
import ConfirmationBox from "../../Helpers/ConfirmationBox";

import "./Libraries.scss";

function Libraries(props) {
  const [libs, setLibs] = useState([]);

  const handle_ws_msg = async ({data}) => {
    const payload = JSON.parse(data);

    switch(payload.type) {
      case "EventRemoveLibrary":
        props.handleWsDelLibrary(payload.id);
        break;
      case "EventNewLibrary":
        props.handleWsNewLibrary(props.auth.token, payload.id);
        break;
      default:
        break;
    }
  };

  useEffect(() => {
    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);

    if (window.location.protocol !== "https:") {
      library_ws.addEventListener("message", handle_ws_msg);
    }

    props.fetchLibraries(props.auth.token);

    return () => {
      library_ws.removeEventListener("message", handle_ws_msg);
      library_ws.close();
    };
  }, []);

  useEffect(() => {
    setLibs(props.libraries.items);
  }, [props.libraries.items]);

  return (
    <section className="libraries">
      <header>
        <h4>Libraries</h4>
        <NewLibraryModal/>
      </header>
      <div className="list">
        <div className="item-wrapper">
          <NavLink to="/" exact>
            <SidebarIcon icon="dashboard"/>
            <p className="item-wrapper-name">Dashboard</p>
          </NavLink>
        </div>
        {/* FETCH_LIBRARIES_OK */}
        {(props.libraries.fetched && !props.libraries.error && libs.length > 0) && (
          libs.map((
            { name, id, media_type }, i
          ) => (
            <div className="item-wrapper" key={i}>
              <NavLink to={"/library/" + id}>
                <SidebarIcon icon={media_type || name}/>
                <p className="item-wrapper-name">{name}</p>
              </NavLink>
              <ConfirmationBox
                action="delete"
                message={`Delete library '${name}'.`}
                continue={() => {
                  props.delLibrary(props.auth.token, id);
                }}/>
            </div>
          ))
        )}
      </div>
    </section>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
  libraries: state.library.fetch_libraries
});

const mapActionsToProps = {
  fetchLibraries,
  delLibrary,
  handleWsDelLibrary,
  handleWsNewLibrary
};

export default connect(mapStateToProps, mapActionsToProps)(Libraries);
