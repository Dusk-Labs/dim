/* eslint-disable no-unused-vars */

import { NavLink } from "react-router-dom";

import FilmIcon from "../../assets/Icons/Film";
import TvIcon from "../../assets/Icons/TvIcon";
import Ring from "../Load/Ring";
import BarLoad from "../Load/Bar";
import { useCallback, useEffect, useState } from "react";

// TODO: show progress loading when scanning lib
function Library(props) {
  const [WS, setWS] = useState(false);
  const [scanning, setScanning] = useState(false);

  const handleWS = useCallback(e => {
    const { type } = JSON.parse(e.data);

    if (type === "EventRemoveLibrary") {
    }
  }, []);

  useEffect(() => {
    const library_ws = new WebSocket(`ws://${window.location.hostname}:3012/events/library`);
    setWS(library_ws);
    return () => library_ws.close();
  }, []);

  const { id, media_type, name } = props;

  return (
    <NavLink
      to={"/library/" + id}
      className="item"
    >
      {media_type === "movie" && <FilmIcon/>}
      {media_type === "tv" && <TvIcon/>}
      <p>{name}</p>
      {scanning && (
        <>
          <Ring small={true}/>
          <BarLoad/>
        </>
      )}
    </NavLink>
  );
}

export default Library;
