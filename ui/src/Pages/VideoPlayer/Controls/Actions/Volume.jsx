import { useCallback, useEffect, useRef, useState, useContext } from "react";
import { useDispatch, useSelector } from "react-redux";
import { VideoPlayerContext } from "../../Context";

import VolumeUpIcon from "../../../../assets/Icons/VolumeUp";
import VolumeMuteIcon from "../../../../assets/Icons/VolumeMute";

import { updateVideo } from "../../../../actions/video";
import { UnfocusableButton } from "Components/unfocusableButton";

function VideoActionVolume() {
  const dispatch = useDispatch();
  const { player } = useContext(VideoPlayerContext);

  const { video } = useSelector((store) => ({
    video: store.video,
  }));

  const volSliderRef = useRef(null);
  const volRef = useRef(null);

  const [currentVolume, setCurrentVolume] = useState(100);
  const [dragging, setDragging] = useState(false);
  const [showVolCount, setShowVolCount] = useState(false);

  const toggleMute = useCallback(() => {
    dispatch(
      updateVideo({
        idleCount: 0,
      })
    );

    if (currentVolume === 0) {
      player.setMute(false);
      player.setVolume(1);
      setCurrentVolume(100);
    }

    if (currentVolume > 0) {
      const currentMuteState = player.isMuted();

      player.setMute(!currentMuteState);

      dispatch(
        updateVideo({
          muted: !currentMuteState,
        })
      );
    }
  }, [currentVolume, dispatch, player]);

  const handleKeyDown = useCallback(
    (e) => {
      if (e.key === "m") {
        toggleMute();
      }
    },
    [toggleMute]
  );

  const handleClick = useCallback((e) => {
    const slider = volSliderRef.current.getBoundingClientRect();
    const x = e.clientX - slider.left;

    let percent = Math.round((x / slider.width) * 100);

    if (percent <= 10) {
      percent = 0;
    }

    if (percent >= 90) {
      percent = 100;
    }

    volRef.current.style.transition = "width 100ms ease-in-out";

    setCurrentVolume(percent);
    setShowVolCount(true);
  }, []);

  const handleMouseDown = useCallback(() => {
    setDragging(true);
  }, []);

  const handleMouseUp = useCallback(() => {
    setDragging(false);
  }, []);

  const handleMouseMove = useCallback(
    (e) => {
      if (dragging) {
        const slider = volSliderRef.current.getBoundingClientRect();
        const x = e.clientX - slider.left;

        let percent = Math.round((x / slider.width) * 100);

        if (percent <= 5) {
          percent = 0;
        }

        if (percent >= 95) {
          percent = 100;
        }

        volRef.current.style.transition = "";

        setCurrentVolume(percent);
        setShowVolCount(true);
      }
    },
    [dragging]
  );

  const handleWheelEvent = useCallback(
    (e) => {
      if (e.deltaY > 0) {
        let newVolume = currentVolume - 5;
        if (newVolume < 0) {
          newVolume = 0;
        }
        setCurrentVolume(newVolume);
      } else if (e.deltaY < 0) {
        let newVolume = currentVolume + 5;
        if (newVolume > 100) {
          newVolume = 100;
        }
        setCurrentVolume(newVolume);
      }
    },
    [currentVolume]
  );

  useEffect(() => {
    localStorage.setItem("videoVolume", currentVolume);
  }, [currentVolume]);

  useEffect(() => {
    if (!player) return;

    const newVolume = currentVolume / 100;

    player.setVolume(newVolume);

    volRef.current.style.width = `${currentVolume}%`;
    volSliderRef.current.setAttribute("data-currentVolume", currentVolume);
  }, [currentVolume, player]);

  useEffect(() => {
    if (!player) return;

    const prefVideoVolume = localStorage.getItem("videoVolume");

    if (prefVideoVolume) {
      setCurrentVolume(prefVideoVolume);
      player.setVolume(prefVideoVolume / 100);
    }

    if (!prefVideoVolume) {
      setCurrentVolume(100);
      player.setVolume(prefVideoVolume / 100);
    }
  }, [player]);

  useEffect(() => {
    const volSlider = volSliderRef.current;

    volSlider.addEventListener("click", handleClick);
    volSlider.addEventListener("mousedown", handleMouseDown);
    volSlider.addEventListener("mouseup", handleMouseUp);
    volSlider.addEventListener("mousemove", handleMouseMove);

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("wheel", handleWheelEvent);

    return () => {
      volSlider.removeEventListener("click", handleClick);
      volSlider.removeEventListener("mousedown", handleMouseDown);
      volSlider.removeEventListener("mouseup", handleMouseUp);
      volSlider.removeEventListener("mousemove", handleMouseMove);

      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("wheel", handleWheelEvent);
    };
  }, [
    handleClick,
    handleKeyDown,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,
    handleWheelEvent,
  ]);

  return (
    <>
      <UnfocusableButton onClick={toggleMute} className="volume">
        {!video.muted && currentVolume > 0 ? (
          <VolumeUpIcon />
        ) : (
          <VolumeMuteIcon />
        )}
      </UnfocusableButton>
      <div className="volSliderWrapper">
        <div
          className={`volSlider dragging-${showVolCount}`}
          ref={volSliderRef}
        >
          <div className="vol" ref={volRef} />
        </div>
      </div>
    </>
  );
}

export default VideoActionVolume;
