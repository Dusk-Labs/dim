import {
    QUICK_SEARCH_START,
    QUICK_SEARCH_OK,
    QUICK_SEARCH_ERR,
    SEARCH_START,
    SEARCH_OK,
    SEARCH_ERR
} from "./types.js";

export const search = (params) => async (dispatch) => {
    dispatch({ type: SEARCH_START });

    try {
        const res = await fetch(`http://${window.host}:8000/api/v1/search?${params}`);

        if (res.status !== 200) {
            return dispatch({
                type: SEARCH_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: SEARCH_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: SEARCH_ERR,
            payload: err
        });
    }
};

export const quickSearch = (query) => async (dispatch) => {
    dispatch({ type: QUICK_SEARCH_START });

    try {
        const res = await fetch(`http://${window.host}:8000/api/v1/search?query=${query}&quick=true`);

        if (res.status !== 200) {
            return dispatch({
                type: QUICK_SEARCH_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: QUICK_SEARCH_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: QUICK_SEARCH_ERR,
            payload: err
        });
    }
};
