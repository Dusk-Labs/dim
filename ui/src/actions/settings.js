import {
  FETCH_USER_SETTINGS_START,
  FETCH_USER_SETTINGS_OK,
  FETCH_USER_SETTINGS_ERR,
  FETCH_GLOBAL_SETTINGS_START,
  FETCH_GLOBAL_SETTINGS_OK,
  FETCH_GLOBAL_SETTINGS_ERR,
  UPDATE_USER_SETTINGS,
  UPDATE_GLOBAL_SETTINGS,
} from "./types";

import { addNotification } from "../slices/notifications";

export const fetchUserSettings = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_USER_SETTINGS_START });

  const config = {
    headers: {
      Authorization: token,
    },
  };

  try {
    const res = await fetch("/api/v1/user/settings", config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_USER_SETTINGS_ERR,
        payload: res.statusText,
      });
    }

    const settings = await res.json();

    dispatch({
      type: FETCH_USER_SETTINGS_OK,
      payload: settings,
    });
  } catch (err) {
    dispatch({
      type: FETCH_USER_SETTINGS_ERR,
      payload: err,
    });
  }
};

export const fetchGlobalSettings = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({
    type: FETCH_GLOBAL_SETTINGS_START,
  });

  const config = {
    headers: {
      authorization: token,
    },
  };

  try {
    const res = await fetch("/api/v1/host/settings", config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_GLOBAL_SETTINGS_ERR,
        payload: res.statusText,
      });
    }

    const settings = await res.json();

    dispatch({
      type: FETCH_GLOBAL_SETTINGS_OK,
      payload: settings,
    });
  } catch (err) {
    dispatch({
      type: FETCH_GLOBAL_SETTINGS_ERR,
      payload: err,
    });
  }
};

export const updateUserSettings = (data) => async (dispatch, getState) => {
  const state = getState();

  const userSettings = state.settings.userSettings;
  const token = state.auth.token;

  const newSettings = {
    ...userSettings.data,
    ...data,
  };

  const config = {
    method: "POST",
    headers: {
      Authorization: token,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(newSettings),
  };

  try {
    const res = await fetch("/api/v1/user/settings", config);

    if (res.status !== 200) {
      dispatch(
        addNotification({
          msg: "Failed to save settings.",
        })
      );

      return;
    }

    dispatch({
      type: UPDATE_USER_SETTINGS,
      payload: newSettings,
    });

    dispatch(
      addNotification({
        msg: "Successfuly saved your changes.",
      })
    );
  } catch (err) {
    dispatch(
      addNotification({
        msg: "Failed to save settings.",
      })
    );
  }
};

export const updateGlobalSettings = (data) => async (dispatch, getState) => {
  const state = getState();

  const globalSettings = state.settings.globalSettings.data;
  const token = state.auth.token;

  const newSettings = {
    ...globalSettings,
    ...data,
  };

  const config = {
    method: "POST",
    headers: {
      Authorization: token,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(newSettings),
  };

  try {
    const res = await fetch("/api/v1/host/settings", config);

    if (res.status !== 200) {
      dispatch(
        addNotification({
          msg: "Failed to save settings.",
        })
      );

      return;
    }

    dispatch({
      type: UPDATE_GLOBAL_SETTINGS,
      payload: newSettings,
    });

    dispatch(
      addNotification({
        msg: "Successfuly saved your changes.",
      })
    );
  } catch (err) {
    dispatch(
      addNotification({
        msg: "Failed to save settings.",
      })
    );
  }
};
