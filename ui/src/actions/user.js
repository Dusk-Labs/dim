import {
  FETCH_USER_START,
  FETCH_USER_OK,
  FETCH_USER_ERR,
  NOTIFICATIONS_ADD,
  CHANGE_USERNAME_START,
  CHANGE_USERNAME_ERR,
  CHANGE_USERNAME_OK,
  CHANGE_AVATAR_START,
  CHANGE_AVATAR_ERR,
  CHANGE_AVATAR_OK
} from "./types.js";

export const fetchUser = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_USER_START });

  const config = {
    headers: {
      "Authorization": token
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

export const changeUsername = (newUsername) => async (dispatch, getState) => {
  dispatch({ type: CHANGE_USERNAME_START });

  const token = getState().auth.token;

  const config = {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
      "authorization": token
    },
    body: JSON.stringify({
      new_username: newUsername
    })
  };

  try {
    const res = await fetch("/api/v1/auth/username", config);

    if (res.status !== 200) {
      dispatch({
        type: CHANGE_USERNAME_ERR,
        payload: res.statusText
      });

      return;
    }

    dispatch({ type: CHANGE_USERNAME_OK });

    dispatch({
      type: NOTIFICATIONS_ADD,
      payload: {
        msg: "Your username has now been updated."
      }
    });
  } catch(err) {
    dispatch({
      type: CHANGE_USERNAME_ERR,
      payload: err
    });
  }
};

export const changeAvatar = (file) => async (dispatch, getState) => {
  dispatch({ type: CHANGE_AVATAR_START });

  const token = getState().auth.token;

  const config = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "authorization": token
    },
    body: JSON.stringify(file)
  };

  try {
    const res = await fetch("/api/v1/user/avatar", config);

    if (res.status !== 200) {
      dispatch({
        type: CHANGE_AVATAR_ERR,
        payload: res.statusText
      });

      return;
    }

    dispatch({ type: CHANGE_AVATAR_OK });

    dispatch({
      type: NOTIFICATIONS_ADD,
      payload: {
        msg: "Your new picture has now been set as your avatar."
      }
    });
  } catch(err) {
    dispatch({
      type: CHANGE_AVATAR_ERR,
      payload: err
    });
  }
};
