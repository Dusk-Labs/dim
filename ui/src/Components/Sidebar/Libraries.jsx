import { useCallback, useEffect } from "react";
import { NavLink } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import NewLibraryModal from "../../Modals/NewLibrary/Index";

import { fetchLibraries, handleWsNewLibrary, handleWsDelLibrary, wsScanStart, wsScanStop } from "../../actions/library.js";

import HomeIcon from "../../assets/Icons/Home";
import Library from "./Library";

function Libraries() {
  const dispatch = useDispatch();

  const { ws, libraries } = useSelector(store => ({
    libraries: store.library.fetch_libraries,
    ws: store.ws
  }));

  const handleWS = useCallback(async ({data}) => {
    const payload = JSON.parse(data);

    if (payload.type === "EventStartedScanning") {
      dispatch(wsScanStart(payload.id));
    }

    if (payload.type === "EventStoppedScanning") {
      dispatch(wsScanStop(payload.id));
    }

    if (payload.type === "EventNewLibrary") {
      dispatch(handleWsNewLibrary(payload.id));
    }

    if (payload.type === "EventRemoveLibrary") {
      dispatch(handleWsDelLibrary(payload.id));
    }
  }, [dispatch]);

  useEffect(() => {
    if (!ws.conn) return;

    ws.conn.addEventListener("message", handleWS);
    return () => ws.conn.removeEventListener("message", handleWS);
  }, [handleWS, ws.conn]);

  useEffect(() => {
    dispatch(fetchLibraries());
  }, [dispatch]);

  let libs;

  const { fetched, error, items } = libraries;

  // FETCH_LIBRARIES_OK
  if (fetched && !error && items.length > 0) {
    libs = items.map((props, i) => (
      <Library {...props} key={i}/>
    ));
  }

  return (
    <section className="libraries">
      <header>
        <h4>Libraries</h4>
        <NewLibraryModal libCount = {items.length}>
          <button className="openNewLibrary">
            +
          </button>
        </NewLibraryModal>
      </header>
      <div className="list">
        <NavLink className="item" to="/" exact>
          <HomeIcon/>
          <p>Dashboard</p>
        </NavLink>
        {libs}
      </div>
    </section>
  );
}

export default Libraries;
