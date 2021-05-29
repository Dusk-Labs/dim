import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import Volume from "./Actions/Volume";
import SeekBack from "./Actions/SeekBack";
import PlayPause from "./Actions/PlayPause";
import SeekForward from "./Actions/SeekForward";
import Subtitles from "./Actions/Subtitles";
import Fullscreen from "./Actions/Fullscreen";

import { updateVideo } from "../../../actions/video";

import "./Actions.scss";

function VideoActions(props) {
  const dispatch = useDispatch();

  const { video } = useSelector(store => ({
    video: store.video
  }));

  const { setVisible } = props;

  useEffect(() => {
    const body = document.getElementsByTagName("body")[0];
    setVisible(video.idleCount <= 2);
    body.style.cursor = video.idleCount <= 2 ? "default" : "none";
  }, [video.idleCount, setVisible]);

  const showPlayer = useCallback(() => {
    dispatch(updateVideo({
      idleCount: 0
    }));

    setVisible(true);
    document.getElementsByTagName("body")[0].style.cursor = "default";
  }, [dispatch, setVisible]);

  useEffect(() => {
    document.addEventListener("mousemove", showPlayer);

    return () => {
      document.removeEventListener("mousemove", showPlayer);
    };
  }, [showPlayer]);

  return (
    <div className="videoActions">
      <section className="left">
        <Volume/>
      </section>
      <section className="middle">
        <SeekBack/>
        <PlayPause/>
        <SeekForward/>
      </section>
      <section className="right">
        <Subtitles/>
        <Fullscreen/>
      </section>
    </div>
  );
}

export default VideoActions;
