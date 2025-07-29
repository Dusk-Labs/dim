import { NavLink } from "react-router-dom";

import { useAppSelector } from "hooks/store";
import FilmIcon from "assets/Icons/Film";
import TvIcon from "assets/Icons/TvIcon";
import BarLoad from "Components/Load/Bar";

interface Props {
  id: string;
  media_type: string;
  name: string;
}

function Library(props: Props) {
  const scanning = useAppSelector((store) => store.library.scanning);
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
