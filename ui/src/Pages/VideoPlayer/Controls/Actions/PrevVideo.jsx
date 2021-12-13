import { useCallback, useEffect, useState } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import PrevVideoIcon from "../../../../assets/Icons/PrevVideo";

import { fetchMediaFiles } from "../../../../actions/media";

function VideoActionPrevVideo() {
  const { media, video } = useSelector(store => ({
    media: store.media,
    video: store.video
  }));

  const [enabled, setEnable] = useState(false);

  const history = useHistory();
  const dispatch = useDispatch();
  const currentMedia = media[video.mediaID];

  useEffect(() => {
    const prevVideoId = currentMedia?.info?.data?.prev_episode_id;
    if(!prevVideoId) return;

    setEnable(true);
    dispatch(fetchMediaFiles(prevVideoId));
  }, [dispatch, currentMedia, setEnable]);

  const nextVideo = useCallback(() => {
    if(!video.mediaID) return;

    const prevVideoId = media[video.mediaID]?.info?.data.prev_episode_id;
    const item = media[prevVideoId]?.files?.items[0];

    if (item) {
      if (history.location.state?.from && history.location.state.from.startsWith("/play")) {
        history.replace(`/play/${item.id}`, { from: history.location.pathname });
      } else {
        history.push(`/play/${item.id}`, { from: history.location.pathname });
      }
    }
  }, [history, video.mediaID, media]);

  return (
    <button onClick={nextVideo} className={`prev_video ${enabled}`} disabled={!enabled}>
      <PrevVideoIcon/>
    </button>
  );
}

export default VideoActionPrevVideo;
