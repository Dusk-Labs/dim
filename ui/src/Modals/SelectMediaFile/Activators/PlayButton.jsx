import { useCallback, useContext } from "react";

import Button from "../../../Components/Misc/Button";
import PlayIcon from "../../../assets/Icons/Play";
import { SelectMediaFileContext } from "../Context";

function SelectMediaFilePlayButton(props) {
  const { setClicked, currentID } = useContext(SelectMediaFileContext);
  const { progress, seasonep, label, hideIcon } = props;

  const handleClick = useCallback(() => {
    if (!currentID) return;

    setClicked(true);
  }, [currentID, setClicked]);

  const name = seasonep?.season
    ? `S${seasonep.season} E${seasonep.episode}`
    : "movie";

  return (
    <Button type="icon" onClick={handleClick}>
      {label ? (
        <p>{label}</p>
      ) : (
        <p>
          {progress > 0 ? "Resume" : "Play"} {name}
        </p>
      )}
      {!hideIcon && <PlayIcon />}
    </Button>
  );
}

export default SelectMediaFilePlayButton;
