import { useCallback, useContext, useEffect, useRef } from "react";
import { useDispatch } from "react-redux";
import { skipToken } from "@reduxjs/toolkit/query/react";

import { useGetMediaQuery } from "../../../api/v1/media";

import { SelectMediaFileContext } from "../Context";
import CardImage from "../../../Pages/Media/CardImage";

function SelectMediaFileEpisode(props) {
  const dispatch = useDispatch();
  const epRef = useRef(null);

  const { setClicked, currentID } = useContext(SelectMediaFileContext);
  const { number, thumbnail, onHover } = props;

  const { data } = useGetMediaQuery(currentID ? currentID : skipToken);

  const handleClick = useCallback(() => {
    if (!currentID) return;

    setClicked(true);
  }, [currentID, setClicked]);

  const handleMouseEnter = useCallback(() => {
    onHover();
  }, [onHover]);

  useEffect(() => {
    if (!currentID) return;

    const current = epRef.current;

    if (current) {
      current.addEventListener("mouseenter", handleMouseEnter);

      return () => {
        current.removeEventListener("mouseenter", handleMouseEnter);
      };
    }
  }, [currentID, dispatch, handleMouseEnter, epRef]);

  let progressPercentage = 0;

  if (data) {
    const { progress, duration } = data;

    progressPercentage = (progress / duration) * 100;
  }

  return (
    <div className="episode" onClick={handleClick} ref={epRef}>
      <CardImage src={thumbnail} progress={progressPercentage} />
      <p>Episode {number}</p>
    </div>
  );
}

export default SelectMediaFileEpisode;
