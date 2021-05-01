/* eslint-disable no-unused-vars */

import { NavLink } from "react-router-dom";

import FilmIcon from "../../assets/Icons/Film";
import TvIcon from "../../assets/Icons/TvIcon";
import Ring from "../Load/Ring";
import BarLoad from "../Load/Bar";
import { useSelector } from "react-redux";

// TODO: show progress loading when scanning lib
function Library(props) {
  const status = useSelector(store => store.library.scan_progress);
  const { id, media_type, name } = props;

  return (
    <NavLink
      to={"/library/" + id}
      className="item"
    >
      {media_type === "movie" && <FilmIcon/>}
      {media_type === "tv" && <TvIcon/>}
      <p>{name}</p>
      {status[id] && (
        <>
          <Ring small={true}/>
          <BarLoad/>
        </>
      )}
    </NavLink>
  );
}

export default Library;
