import {
    FETCH_CARDS_START,
    FETCH_CARDS_OK,
    FETCH_CARDS_ERR
} from "./types.js";

export const fetchCards = (path) => async (dispatch) => {
    dispatch({ type: FETCH_CARDS_START });

    try {
        const res = await fetch(path);

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_DIRECTORIES_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: FETCH_CARDS_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: FETCH_CARDS_ERR,
            payload: err
        });
    }
}