import { useCallback, useEffect, useRef } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateTrack, updateVideo } from "../../../actions/video";

function VideoMenuSubSwitcher() {
  const dispatch = useDispatch();

  const { video, subtitleTracks } = useSelector((store) => ({
    video: store.video,
    subtitleTracks: store.video.tracks.subtitle,
  }));

  const menuRef = useRef(null);

  const changeTrack = useCallback(
    (i) => {
      console.log(subtitleTracks.current, i);

      if (subtitleTracks.current === i) return;

      dispatch(
        updateTrack("subtitle", {
          current: i,
          ready: false,
        })
      );

      dispatch(
        updateVideo({
          textTrackEnabled: true,
          showSubSwitcher: false,
        })
      );
    },
    [dispatch, subtitleTracks]
  );

  const turnOffSubs = useCallback(() => {
    if (subtitleTracks.current === -1) return;
    console.log("[Subtitles] turning off subs");

    dispatch(
      updateVideo({
        textTrackEnabled: false,
      })
    );

    dispatch(
      updateTrack("subtitle", {
        current: -1,
        ready: false,
      })
    );
  }, [dispatch, subtitleTracks]);

  const handleClick = useCallback(
    (e) => {
      if (!menuRef.current || e.target.nodeName !== "DIV") return;

      if (!menuRef.current.contains(e.target)) {
        dispatch(
          updateVideo({
            showSubSwitcher: false,
          })
        );
      }
    },
    [dispatch]
  );

  useEffect(() => {
    if (video.idleCount >= 2) {
      dispatch(
        updateVideo({
          showSubSwitcher: false,
        })
      );
    }
  }, [video.idleCount, dispatch]);

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    };
  }, [handleClick]);

  return (
    <div className="menu" ref={menuRef}>
      <div className="heading">
        <h3>Subtitles</h3>
      </div>
      <div className="separatorContainer">
        <div className="separator" />
      </div>
      <div className="tracks">
        <div
          className={`track ${subtitleTracks.current === -1 ? "active" : ""}`}
          onClick={turnOffSubs}
        >
          <p>Off</p>
        </div>
        {subtitleTracks.list.map((track, i) => (
          <div
            key={i}
            className={`track ${subtitleTracks.current === i ? "active" : ""}`}
            onClick={() => changeTrack(i)}
          >
            <p>{track.title || "No title"}</p>
          </div>
        ))}
      </div>
    </div>
  );
}

export default VideoMenuSubSwitcher;
