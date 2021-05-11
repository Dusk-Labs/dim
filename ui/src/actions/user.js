import {
  FETCH_USER_START,
  FETCH_USER_OK,
  FETCH_USER_ERR
} from "./types.js";

export const fetchUser = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_USER_START });

  const config = {
    headers: {
      "Authorization": token,
    }
  };

  try {
    const res = await fetch("/api/v1/auth/whoami", config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_USER_ERR,
        payload: res.statusText
      });
    }

    const profile = await res.json();

    dispatch({
      type: FETCH_USER_OK,
      payload: profile
    });
  } catch(err) {
    dispatch({
      type: FETCH_USER_ERR,
      payload: err
    });
  }
};
