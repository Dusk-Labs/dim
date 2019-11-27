import {
    FETCH_BANNERS_START,
    FETCH_BANNERS_OK,
    FETCH_BANNERS_ERR
} from "./types.js";

export const fetchBanners = () => async (dispatch) => {
    dispatch({ type: FETCH_BANNERS_START });

    try {
        const res = await fetch("http://86.21.150.167:8000/api/v1/dashboard/banner");

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_BANNERS_ERR,
                payload: res.statusText
            });
        }

        const payload = await res.json();

        dispatch({
            type: FETCH_BANNERS_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: FETCH_BANNERS_ERR,
            payload: err
        });
    }
};