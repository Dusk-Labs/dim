import React, { useEffect } from "react";
import { NavLink } from "react-router-dom";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchLibraries, handleWsNewLibrary, handleWsDelLibrary } from "../../actions/library.js";
import NewLibraryModal from "./NewLibraryModal/Index.jsx";

function Libraries(props) {
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
        <NewLibraryModal/>
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
