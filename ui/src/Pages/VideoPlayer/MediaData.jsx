import { useEffect } from "react";
import { useParams } from "react-router";
import { useDispatch, useSelector } from "react-redux";

import { updateVideo } from "../../actions/video";
import { clearMediaInfo, fetchExtraMediaInfo, fetchMediaInfo } from "../../actions/card";

function VideoMediaData() {
  const params = useParams();
  const dispatch = useDispatch();

  const { video, auth, extra_media_info } = useSelector(store => ({
    auth: store.auth,
    video: store.video,
    extra_media_info: store.card.extra_media_info
  }));

  const { token } = auth;

  useEffect(() => {
    (async () => {
      const config = {
        headers: {
          "authorization": token
        }
      };

      const res = await fetch(`/api/v1/mediafile/${params.fileID}`, config);

      if (res.status !== 200) {
        return;
      }

      const payload = await res.json();

      dispatch(updateVideo({
        mediaID: payload.media_id
      }));
    })();
  }, [dispatch, params.fileID, token]);

  useEffect(() => {
    if (extra_media_info.info.seasons) {
      const { seasons } = extra_media_info.info;

      let episode;

      for (const season of seasons) {
        const found = season.episodes.filter(ep => {
          return ep.versions.filter(version => version.id === parseInt(params.fileID)).length === 1;
        });

        if (found.length > 0) {
          episode = {
            ...found[0],
            season: season.season_number
          };

          break;
        }
      }

      if (episode) {
        dispatch(updateVideo({episode}));
      }
    }
  }, [dispatch, extra_media_info.info, params.fileID]);

  useEffect(() => {
    if (!video.mediaID) return;
    dispatch(fetchExtraMediaInfo(video.mediaID));
    return () => dispatch(clearMediaInfo());
  }, [dispatch, video.mediaID]);

  useEffect(() => {
    if (!video.mediaID) return;
    dispatch(fetchMediaInfo(video.mediaID));
    return () => dispatch(clearMediaInfo());
  }, [dispatch, video.mediaID]);

  return null;
}

export default VideoMediaData;
