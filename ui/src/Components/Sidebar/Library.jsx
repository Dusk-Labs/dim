import { NavLink } from "react-router-dom";

import FilmIcon from "../../assets/Icons/Film";
import TvIcon from "../../assets/Icons/TvIcon";
import BarLoad from "../Load/Bar";
import { useSelector } from "react-redux";

function Library(props) {
  const scanning = useSelector((store) => store.library.scanning);
  const { id, media_type, name } = props;

  return (
    <NavLink
      to={"/library/" + id}
      className={`item showLoad-${scanning.includes(id)}`}
    >
      {media_type === "movie" && <FilmIcon />}
      {media_type === "tv" && <TvIcon />}
      <p>{name}</p>
      {scanning.includes(id) && <BarLoad />}
    </NavLink>
  );
}

export default Library;
