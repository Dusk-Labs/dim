import {
    FETCH_DASHBOARD_START,
    FETCH_DASHBOARD_OK,
    FETCH_DASHBOARD_ERR,
    FETCH_BANNERS_START,
    FETCH_BANNERS_OK,
    FETCH_BANNERS_ERR
} from "./types.js";

export const fetchDashboard = () => async (dispatch) => {
    dispatch({ type: FETCH_DASHBOARD_START });

    try {
        const res = await fetch("http://86.21.150.167:8000/api/v1/dashboard");

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_DASHBOARD_ERR,
                payload: res.statusText
            });
        }

        const cards = await res.json();

        dispatch({
            type: FETCH_DASHBOARD_OK,
            payload: cards
        });
    } catch(err) {
        dispatch({
            type: FETCH_DASHBOARD_ERR,
            payload: err
        });
    }
};

export const fetchBanners = () => async (dispatch) => {
    dispatch({ type: FETCH_BANNERS_START });

    try {
        const res = await fetch("http://86.21.150.167:8000/api/v1/dashboard/banner");

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_DASHBOARD_OK,
                payload: res.statusText
            });
        }

        const banners = await res.json();

        dispatch({
            type: FETCH_BANNERS_OK,
            payload: banners
        });
    } catch(err) {
        dispatch({
            type: FETCH_BANNERS_ERR,
            payload: err
        });
    }
};