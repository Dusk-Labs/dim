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

  const prevEpisodeId = currentMedia?.info?.data?.prev_episode_id;
  const nextMedia = prevEpisodeId ? media[prevEpisodeId] : null;

  useEffect(() => {
    if(!prevEpisodeId) return;

    setEnable(true);
    dispatch(fetchMediaFiles(prevEpisodeId));
  }, [dispatch, prevEpisodeId]);

  const nextVideo = useCallback(() => {
    const item = nextMedia?.files?.items[0];
    if(!item) return;

    history.replace(`/play/${item.id}`, { from: history.location.pathname });
  }, [history, nextMedia]);

  return (
    <button onClick={nextVideo} className={`prev_video ${enabled}`} disabled={!enabled}>
      <PrevVideoIcon/>
    </button>
  );
}

export default VideoActionPrevVideo;
