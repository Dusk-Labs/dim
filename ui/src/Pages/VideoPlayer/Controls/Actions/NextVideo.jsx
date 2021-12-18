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

  const nextEpisodeId = currentMedia?.info?.data?.next_episode_id;
  const nextMedia = nextEpisodeId ? media[nextEpisodeId] : null;

  useEffect(() => {
    if(!nextEpisodeId) return;

    setEnable(true);
    dispatch(fetchMediaFiles(nextEpisodeId));
  }, [dispatch, nextEpisodeId]);

  const nextVideo = useCallback(() => {
    const item = nextMedia?.files?.items[0];
    if(!item) return;

    history.replace(`/play/${item.id}`, { from: history.location.pathname });
  }, [history, nextMedia]);

  return (
    <button onClick={nextVideo} className={`next_video ${enabled}`} disabled={!enabled}>
      <NextVideoIcon/>
    </button>
  );
}

export default VideoActionNextVideo;
