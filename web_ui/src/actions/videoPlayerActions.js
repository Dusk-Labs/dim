import {
    TRANSCODE_START,
    TRANSCODE_OK,
    TRANSCODE_ERR,
    DEL_TRANSCODE_START,
    DEL_TRANSCODE_OK,
    DEL_TRANSCODE_ERR
} from "./types.js";

export const startTranscode = (id, params = "") => async (dispatch) => {
    dispatch({ type: TRANSCODE_START });

    try {
        const res = await fetch(`http://${window.host}:8000/api/v1/stream/start/${id}${params}`);

        if (res.status !== 200) {
            return dispatch({
                type: TRANSCODE_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: TRANSCODE_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: TRANSCODE_ERR,
            payload: err
        });
    }
};

export const delTranscode = (uuid) => async (dispatch) => {
    dispatch({ type: DEL_TRANSCODE_START });

    const config = {
        method: "DELETE"
    };

    try {
        const res = await fetch(`http://${window.host}:8000/api/v1/stream/${uuid}`, config);

        if (res.status !== 200) {
            return dispatch({
                type: DEL_TRANSCODE_ERR,
                payload: res.statusText
            });
        }

        dispatch({ type: DEL_TRANSCODE_OK });
    } catch(err) {
        dispatch({
            type: DEL_TRANSCODE_ERR
        });
    }
};