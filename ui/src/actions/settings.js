import {
  FETCH_USER_SETTINGS,
  FETCH_USER_SETTINGS_START,
  FETCH_USER_SETTINGS_ERR,
  FETCH_GLOBAL_SETTINGS_START,
  FETCH_GLOBAL_SETTINGS,
  FETCH_GLOBAL_SETTINGS_ERR
} from "./types.js";

export const fetchUserSettings = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_USER_SETTINGS_START });

  const config = {
    headers: {
      "Authorization" : token
    }
  };

  try {
    const res = await fetch("/api/v1/user/settings", config);

    if (res.status !== 200) {
      return dispatch({ type: FETCH_USER_SETTINGS_ERR, payload: res.statusText });
    }

    const settings = await res.json();

    dispatch({type: FETCH_USER_SETTINGS, payload: settings});

  } catch(err) {
    dispatch({ type: FETCH_USER_SETTINGS_ERR, payload: err});
  }
};

export const fetchGlobalSettings = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_GLOBAL_SETTINGS_START });

  const config = {
    headers: {
      "Authorization" : token
    }
  };

  try {
    const res = await fetch("/api/v1/host/settings", config);

    if (res.status !== 200) {
      return dispatch({ type: FETCH_GLOBAL_SETTINGS_ERR, payload: res.statusText });
    }

    const settings = await res.json();

    dispatch({type: FETCH_GLOBAL_SETTINGS, payload: settings});

  } catch(err) {
    dispatch({ type: FETCH_GLOBAL_SETTINGS_ERR, payload: err});
  }
};
