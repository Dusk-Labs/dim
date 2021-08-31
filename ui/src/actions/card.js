import {
  FETCH_CARDS_START,
  FETCH_CARDS_OK,
  FETCH_CARDS_ERR
} from "./types.js";

export const fetchCards = (path, reset = true) => async (dispatch, getState) => {
  const token = getState().auth.token;

  if (reset) {
    dispatch({ type: FETCH_CARDS_START });
  }

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(path, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_CARDS_ERR,
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
};
