import { useCallback, useEffect, useRef, useState, useContext } from "react";
import { useDispatch, useSelector } from "react-redux";

import { updateVideo, updateTrack } from "../../../actions/video";

import { VideoPlayerContext } from "../Context";

import ArrowLeftIcon from "../../../assets/Icons/ArrowLeft";
import ChevronRightIcon from "../../../assets/Icons/ChevronRight";

function VideoMenuSettings() {
  const dispatch = useDispatch();

  const { player } = useContext(VideoPlayerContext);

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const [activeInnerMenu, setActiveInnerMenu] = useState();

  const menuRef = useRef(null);

  const handleClick = useCallback(
    (e) => {
      if (!menuRef.current || e.target.nodeName !== "DIV") return;

      if (!menuRef.current.contains(e.target)) {
        dispatch(
          updateVideo({
            showSettings: false,
          })
        );
      }
    },
    [dispatch]
  );

  const goBack = useCallback(() => {
    if (!activeInnerMenu) return;
    setActiveInnerMenu();
  }, [activeInnerMenu]);

  const changeTrack = useCallback(
    (trackType, i) => {
      const tracks =
        trackType === "video"
          ? video.tracks.video.list
          : video.tracks.audio.list;

      const playerTracks = player.getTracksFor(trackType);
      const selectedTrack = playerTracks.filter(
        (track) => track.id === tracks[i].set_id
      );

      console.log("[video] changed track to", selectedTrack[0]);

      player.setCurrentTrack(selectedTrack[0]);

      dispatch(
        updateTrack(trackType, {
          current: parseInt(i),
        })
      );
    },
    [dispatch, player, video]
  );

  useEffect(() => {
    window.addEventListener("click", handleClick);

    return () => {
      window.removeEventListener("click", handleClick);
    };
  }, [handleClick]);

  window.video = video;
  return (
    <div className="menu" ref={menuRef}>
      <div className="heading">
        <h3>{activeInnerMenu ? activeInnerMenu : "Settings"}</h3>
        {activeInnerMenu && (
          <button onClick={goBack}>
            <ArrowLeftIcon />
          </button>
        )}
      </div>
      <div className="separatorContainer">
        <div className="separator" />
      </div>
      {activeInnerMenu === undefined && (
        <div className="innerMenus">
          <p onClick={() => setActiveInnerMenu("Video Quality")}>
            Video tracks
            <ChevronRightIcon />
          </p>
          <p onClick={() => setActiveInnerMenu("Audio tracks")}>
            Audio tracks
            <ChevronRightIcon />
          </p>
        </div>
      )}
      {activeInnerMenu === "Video Quality" && (
        <div className="innerMenu">
          <div className="tracks">
            {video.tracks.video.list.map((track, i) => (
              <div
                key={i}
                className={`track ${
                  video.tracks.video.current === i ? "active" : ""
                }`}
                onClick={() => changeTrack("video", `${i}`)}
              >
                <p>{track.label}</p>
              </div>
            ))}
          </div>
        </div>
      )}
      {activeInnerMenu === "Audio tracks" && (
        <div className="innerMenu">
          <div className="tracks">
            {video.tracks.audio.list.map((track, i) => (
              <div
                key={i}
                className={`track ${
                  video.tracks.audio.current === i ? "active" : ""
                }`}
                onClick={() => changeTrack("audio", `${i}`)}
              >
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
