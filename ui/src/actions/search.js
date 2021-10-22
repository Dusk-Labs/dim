import {
  QUICK_SEARCH_OK,
  QUICK_SEARCH_ERR,
  SEARCH_START,
  SEARCH_OK,
  SEARCH_ERR
} from "./types";

export const search = (params) => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: SEARCH_START });

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`/api/v1/search${params}`, config);

    if (res.status === 404) {
      return dispatch({
        type: SEARCH_OK,
        payload: []
      });
    }

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

export const quickSearch = (query) => async (dispatch, getState) => {
  const token = getState().auth.token;

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`/api/v1/search?query=${query}&quick=true`, config);

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
