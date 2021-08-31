import { useCallback, useContext } from "react";
import { useDispatch } from "react-redux";

import Button from "../../../Components/Misc/Button";
import PlayIcon from "../../../assets/Icons/Play";
import { fetchMediaFiles } from "../../../actions/media";
import { SelectMediaFileContext } from "../Context";

function SelectMediaFilePlayButton(props) {
  const dispatch = useDispatch();

  const { setClicked, currentID } = useContext(SelectMediaFileContext);
  const { progress, seasonep } = props;

  const handleClick = useCallback(() => {
    if (!currentID) return;

    dispatch(fetchMediaFiles(currentID));
    setClicked(true);
  }, [currentID, dispatch, setClicked]);

  const name = (
    seasonep?.season ? `S${seasonep.season} E${seasonep.episode}` : "movie"
  );

  return (
    <Button type="icon" onClick={handleClick}>
      <p>{progress > 0 ? "Resume" : "Play"} {name}</p>
      <PlayIcon/>
    </Button>
  );
}

export default SelectMediaFilePlayButton;
