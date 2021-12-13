import { useCallback, useEffect, useState } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import NextVideoIcon from "../../../../assets/Icons/NextVideo";

import { fetchMediaFiles } from "../../../../actions/media";

function VideoActionNextVideo() {
  const { media, video } = useSelector(store => ({
    media: store.media,
    video: store.video
  }));

  const [enabled, setEnable] = useState(false);

  const history = useHistory();
  const dispatch = useDispatch();
  const currentMedia = media[video.mediaID];

  useEffect(() => {
    const videoId = currentMedia?.info?.data?.next_episode_id;
    if(!videoId) return;

    setEnable(true);
    dispatch(fetchMediaFiles(videoId));
  }, [dispatch, video.mediaID, currentMedia]);

  const nextVideo = useCallback(() => {
    if(!video.mediaID) return;

    const nextVideoId = media[video.mediaID]?.info?.data.next_episode_id;
    const item = media[nextVideoId]?.files?.items[0];

    if (item) {
      if (history.location.state?.from && history.location.state.from.startsWith("/play")) {
        history.replace(`/play/${item.id}`, { from: history.location.pathname });
      } else {
        history.push(`/play/${item.id}`, { from: history.location.pathname });
      }
    }
  }, [history, video.mediaID, media]);

  return (
    <button onClick={nextVideo} className={`next_video ${enabled}`} disabled={!enabled}>
      <NextVideoIcon/>
    </button>
  );
}

export default VideoActionNextVideo;
