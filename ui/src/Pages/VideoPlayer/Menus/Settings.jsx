import { useCallback, useEffect, useRef, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateVideo, updateTrack } from "../../../actions/video";

import ArrowLeftIcon from "../../../assets/Icons/ArrowLeft";
import ChevronRightIcon from "../../../assets/Icons/ChevronRight";

function VideoMenuSettings() {
  const dispatch = useDispatch();

  const { video } = useSelector(store => ({
    video: store.video
  }));

  const [activeInnerMenu, setActiveInnerMenu] = useState();

  const menuRef = useRef(null);

  const handleClick = useCallback((e) => {
    if (!menuRef.current || e.target.nodeName !== "DIV") return;

    if (!menuRef.current.contains(e.target)) {
      dispatch(updateVideo({
        showSettings: false
      }));
    }
  }, [dispatch]);

  const goBack = useCallback(() => {
    if (!activeInnerMenu) return;
    setActiveInnerMenu();
  }, [activeInnerMenu]);

  const changeTrack = useCallback((track_type, i) => {
    console.log(video.tracks.video.current, i);

    if (video.tracks.video.current === i) return;

    dispatch(updateTrack(track_type, {
      current: i,
      ready: false
    }));
  }, [dispatch, video]);

  useEffect(() => {
    if (video.idleCount >= 2) {
      dispatch(updateVideo({
        showSettings: false
      }));
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
        <h3>{activeInnerMenu ? activeInnerMenu : "Settings"}</h3>
        {activeInnerMenu && (
          <button onClick={goBack}>
            <ArrowLeftIcon/>
          </button>
        )}
      </div>
      <div className="separatorContainer">
        <div className="separator"/>
      </div>
      {activeInnerMenu === undefined && (
        <div className="innerMenus">
          <p onClick={() => setActiveInnerMenu("Video tracks")}>
            Video tracks
            <ChevronRightIcon/>
          </p>
          <p onClick={() => setActiveInnerMenu("Audio tracks")}>
            Audio tracks
            <ChevronRightIcon/>
          </p>
        </div>
      )}
      {activeInnerMenu === "Video tracks" && (
        <div className="innerMenu">
          <div className="tracks">
            {video.tracks.video.list.map((track, i) => (
              <div key={i} className={`track ${video.tracks.video.current === i ? "active" : ""}`} onClick={() => changeTrack("video", i)}>
                <p>{track.label}</p>
              </div>)
            )}
          </div>
        </div>
      )}
      {activeInnerMenu === "Audio tracks" && (
        <div className="innerMenu">
          <div className="tracks">
            {video.tracks.audio.list.map((track, i) => (
              <div key={i} className={`track ${video.tracks.audio.current === i ? "active" : ""}`} onClick={() => changeTrack("audio", i)}>
                <p>{track.label}</p>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

export default VideoMenuSettings;
