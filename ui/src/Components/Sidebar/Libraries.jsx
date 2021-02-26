import React, { useCallback, useEffect } from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import NewLibraryModal from "../../Modals/NewLibrary/Index";

import { fetchLibraries, handleWsNewLibrary, handleWsDelLibrary } from "../../actions/library.js";

function Libraries(props) {
  const { fetchLibraries, handleWsDelLibrary, handleWsNewLibrary, auth } = props;

  const handle_ws_msg = useCallback(async ({data}) => {
    const payload = JSON.parse(data);

    switch(payload.type) {
      case "EventRemoveLibrary":
        handleWsDelLibrary(payload.id);
        break;
      case "EventNewLibrary":
        handleWsNewLibrary(auth.token, payload.id);
        break;
      default:
        break;
    }
  }, [auth.token, handleWsDelLibrary, handleWsNewLibrary]);

  useEffect(() => {
    const library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);

    if (window.location.protocol !== "https:") {
      library_ws.addEventListener("message", handle_ws_msg);
    }

    fetchLibraries(auth.token);

    return () => {
      library_ws.removeEventListener("message", handle_ws_msg);
      library_ws.close();
    };
  }, [auth.token, fetchLibraries, handle_ws_msg]);

  let libraries;

  const { fetched, error, items } = props.libraries;

  // FETCH_LIBRARIES_OK
  if (fetched && !error && items.length > 0) {
    libraries = items.map((
      { name, id, media_type }, i
    ) => (
      <NavLink
        to={"/library/" + id}
        className="item" key={i}
      >
        {media_type === "movie" && <FontAwesomeIcon icon="film"/>}
        {media_type === "tv" && <FontAwesomeIcon icon="tv"/>}
        <p>{name}</p>
      </NavLink>
    ))
  }

  return (
    <section className="libraries">
      <header>
        <h4>Libraries</h4>
        <NewLibraryModal>
          <button className="openNewLibrary">
            +
          </button>
        </NewLibraryModal>
      </header>
      <div className="list">
        <NavLink className="item" to="/" exact>
          <FontAwesomeIcon icon="home"/>
          <p>Dashboard</p>
        </NavLink>
        {libraries}
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
  handleWsDelLibrary,
  handleWsNewLibrary
};

export default connect(mapStateToProps, mapActionsToProps)(Libraries);
