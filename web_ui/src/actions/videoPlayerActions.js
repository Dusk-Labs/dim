import {
    TRANSCODE_START,
    TRANSCODE_OK,
    TRANSCODE_ERR,
    FETCH_FILE_START,
    FETCH_FILE_OK,
    FETCH_FILE_ERR
} from "./types.js";

export const startTranscode = (id) => async (dispatch) => {
    dispatch({ type: TRANSCODE_START });

    try {
        const res = await fetch(`http://86.21.150.167:8000/api/v1/stream/start/${id}`);

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

export const fetchFile = (uuid, file) => async (dispatch) => {
    dispatch({ type: FETCH_FILE_START });

    try {
        const res = await fetch(`http://86.21.150.167:8000/api/v1/stream/static/${uuid}/${file}`);

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_FILE_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: FETCH_FILE_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: FETCH_FILE_ERR,
            payload: err
        });
    }
};