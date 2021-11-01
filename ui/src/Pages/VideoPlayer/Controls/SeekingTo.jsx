import { useCallback, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { formatHHMMSS } from "../../../Helpers/utils";

import "./SeekingTo.scss";

function SeekingTo(props) {
  const { player } = useSelector(store => ({
    player: store.video.player
  }));

  const seekingToDiv = useRef(null);

  const [seekingTo, setSeekingTo] = useState(formatHHMMSS(0));

  const { nameRef, timeRef, seekBar } = props;

  const handleMousemove = useCallback(e => {
    const rect = e.target.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    const videoDuration = player.getMediaElement !== undefined ? player.getMediaElement().duration : player.duration();
    const newTime = Math.floor(percent * videoDuration);

    seekingToDiv.current.style.left = `${e.layerX}px`;

    if (nameRef && timeRef && seekingToDiv.current) {
      const nameRect = nameRef.getBoundingClientRect();
      const timeRect = timeRef.getBoundingClientRect();
      const seekingToRect = seekingToDiv.current.getBoundingClientRect();

      // seekingTo overlapping name from the right side
      if (seekingToRect.left < (nameRect.right + 15)) {
        nameRef.style.opacity = 0;
      } else {
        nameRef.style.opacity = 1;
      }

      // seekingTo overlapping time from the left side
      if (seekingToRect.right > (timeRect.left - 15)) {
        timeRef.style.opacity = 0;
      } else {
        timeRef.style.opacity = 1;
      }
    }

    setSeekingTo(formatHHMMSS(newTime));
  }, [nameRef, player, timeRef]);

  const handleMouseleave = useCallback(() => {
    if (nameRef) {
      nameRef.style.opacity = 1;
    }

    if (timeRef) {
      timeRef.style.opacity = 1;
    }
  }, [nameRef, timeRef]);

  useEffect(() => {
    let bar = seekBar.current;

    bar.addEventListener("mousemove", handleMousemove);
    bar.addEventListener("mouseleave", handleMouseleave);

    return () => {
      bar.removeEventListener("mousemove", handleMousemove);
      bar.removeEventListener("mouseleave", handleMouseleave);
    };
  }, [handleMouseleave, handleMousemove, seekBar]);

  return (
    <div className="seekingTo" ref={seekingToDiv}>
      <p>{seekingTo}</p>
    </div>
  );
}

export default SeekingTo;
