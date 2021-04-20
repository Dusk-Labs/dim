import {
  FETCH_BANNERS_START,
  FETCH_BANNERS_OK,
  FETCH_BANNERS_ERR
} from "./types.js";

export const fetchBanners = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_BANNERS_START });

  try {
    const config = {
      headers: {
        "authorization": token,
      }
    }

    const res = await fetch(`//${window.host}:8000/api/v1/dashboard/banner`, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_BANNERS_ERR,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    // limit to 3 banners
    if (payload.length > 3) {
      payload.length = 3;
    }

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
