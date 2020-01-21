import {
    FETCH_DIRECTORIES_START,
    FETCH_DIRECTORIES_OK,
    FETCH_DIRECTORIES_ERR
} from "./types.js";

export const fetchDirectories = (path, token) => async (dispatch) => {
    dispatch({ type: FETCH_DIRECTORIES_START });

    try {
        const config = {
            headers: {
                "authorization": token,
            }
        }

        const res = await fetch(`http://${window.host}:8000/api/v1/filebrowser/${path}`, config);

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_DIRECTORIES_ERR,
                payload: res.statusText
            });
        }

        const dirs = await res.json();

        dispatch({
            type: FETCH_DIRECTORIES_OK,
            payload: {path, dirs}
        });
    } catch(err) {
        dispatch({
            type: FETCH_DIRECTORIES_ERR,
            payload: err
        });
    }
}
