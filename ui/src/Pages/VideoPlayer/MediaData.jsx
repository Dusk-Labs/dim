import { useEffect } from "react";
import { useParams } from "react-router";
import { useDispatch, useSelector } from "react-redux";

import { updateVideo } from "../../actions/video";
import { fetchMediaInfo } from "../../actions/media";

function VideoMediaData() {
  const params = useParams();
  const dispatch = useDispatch();

  const { video, auth } = useSelector((store) => ({
    auth: store.auth,
    video: store.video,
  }));

  const { token } = auth;

  useEffect(() => {
    (async () => {
      const config = {
        headers: {
          authorization: token,
        },
      };

      const res = await fetch(`/api/v1/mediafile/${params.fileID}`, config);

      if (res.status !== 200) {
        return;
      }

      const payload = await res.json();

      dispatch(
        updateVideo({
          mediaID: payload.media_id,
        })
      );
    })();
  }, [dispatch, params.fileID, token]);

  useEffect(() => {
    if (!video.mediaID) return;
    dispatch(fetchMediaInfo(video.mediaID));
  }, [dispatch, video.mediaID]);

  return null;
}

export default VideoMediaData;
