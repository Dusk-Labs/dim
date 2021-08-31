import { useCallback, useContext, useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { SelectMediaFileContext } from "../Context";
import CardImage from "../../../Pages/Media/CardImage";
import { fetchMediaFiles, fetchMediaInfo } from "../../../actions/media";

function SelectMediaFileEpisode(props) {
  const {media} = useSelector(store => ({
    media: store.media
  }));

  const dispatch = useDispatch();
  const epRef = useRef(null);

  const { setClicked, currentID } = useContext(SelectMediaFileContext);
  const { number, thumbnail } = props;

  const handleClick = useCallback(() => {
    if (!currentID) return;

    dispatch(fetchMediaFiles(currentID));
    setClicked(true);
  }, [currentID, dispatch, setClicked]);

  const handleMouseEnter = useCallback(() => {
    dispatch(fetchMediaInfo(currentID));
  }, [currentID, dispatch]);

  useEffect(() => {
    if (!currentID) return;

    epRef.current.addEventListener("mouseenter", handleMouseEnter);
  }, [currentID, dispatch, handleMouseEnter]);

  let progressPercentage = 0;

  if (media[currentID]) {
    const { progress, duration} = media[currentID].info.data;

    progressPercentage = (
      (progress / duration) * 100
    );
  }

  return (
    <div className="episode" onClick={handleClick} ref={epRef}>
      <CardImage src={thumbnail} progress={progressPercentage}/>
      <p>Episode {number}</p>
    </div>
  );
}

export default SelectMediaFileEpisode;
