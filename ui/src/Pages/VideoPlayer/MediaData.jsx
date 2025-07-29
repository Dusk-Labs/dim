import { useEffect } from "react";
import { useParams } from "react-router";
import { useDispatch, useSelector } from "react-redux";

import { updateVideo } from "../../actions/video";

function VideoMediaData() {
  const params = useParams();
  const dispatch = useDispatch();

  const auth = useSelector((store) => store.auth);

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

  return null;
}

export default VideoMediaData;
