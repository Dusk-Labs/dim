import { useCallback, useEffect } from "react";
import { NavLink } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import NewLibraryModal from "../../Modals/NewLibrary/Index";

import { fetchLibraries, handleWsNewLibrary, handleWsDelLibrary } from "../../actions/library.js";

import HomeIcon from "../../assets/Icons/Home";
import FilmIcon from "../../assets/Icons/Film";
import TvIcon from "../../assets/Icons/TvIcon";

function Libraries() {
  const dispatch = useDispatch();

  const libraries = useSelector(store => (
    store.library.fetch_libraries
  ));

  const handle_ws_msg = useCallback(async ({data}) => {
    const payload = JSON.parse(data);

    switch(payload.type) {
      case "EventRemoveLibrary":
        dispatch(handleWsDelLibrary(payload.id));
        break;
      case "EventNewLibrary":
        dispatch(handleWsNewLibrary(payload.id));
        break;
      default:
        break;
    }
  }, [dispatch]);

  useEffect(() => {
    const library_ws = new WebSocket(`ws://${window.location.hostname}:3012/events/library`);

    if (window.location.protocol !== "https:") {
      library_ws.addEventListener("message", handle_ws_msg);
    }

    dispatch(fetchLibraries());

    return () => {
      library_ws.removeEventListener("message", handle_ws_msg);
      library_ws.close();
    };
  }, [dispatch, handle_ws_msg]);

  let libs;

  const { fetched, error, items } = libraries;

  // FETCH_LIBRARIES_OK
  if (fetched && !error && items.length > 0) {
    libs = items.map((
      { name, id, media_type }, i
    ) => (
      <NavLink
        to={"/library/" + id}
        className="item" key={i}
      >
        {media_type === "movie" && <FilmIcon/>}
        {media_type === "tv" && <TvIcon/>}
        <p>{name}</p>
      </NavLink>
    ));
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
          <HomeIcon/>
          <p>Dashboard</p>
        </NavLink>
        {libs}
      </div>
    </section>
  );
}

export default Libraries;
