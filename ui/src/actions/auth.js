import {
  AUTH_LOGIN_START,
  AUTH_LOGIN_ERR,
  AUTH_LOGIN_OK,
  AUTH_UPDATE_TOKEN,
  AUTH_LOGOUT,
  AUTH_REGISTER_START,
  AUTH_REGISTER_ERR,
  AUTH_REGISTER_OK,
  AUTH_CHECK_ADMIN_ERR,
  AUTH_CHECK_ADMIN_OK,
  CREATE_NEW_INVITE_START,
  CREATE_NEW_INVITE_OK,
  CREATE_NEW_INVITE_ERR,
  FETCH_INVITES_START,
  FETCH_INVITES_OK,
  FETCH_INVITES_ERR,
  DEL_ACCOUNT_START,
  DEL_ACCOUNT_ERR,
  DEL_ACCOUNT_OK,
} from "./types";

import { addNotification } from "../slices/notifications";

export const authenticate = (username, password) => async (dispatch) => {
  dispatch({ type: AUTH_LOGIN_START });

  const config = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      username: username,
      password: password,
    }),
  };

  try {
    const res = await fetch("/api/v1/auth/login", config);

    if (res.status !== 200) {
      return dispatch({
        type: AUTH_LOGIN_ERR,
        payload: res.statusText,
      });
    }

    const payload = await res.json();

    if ("BroadcastChannel" in window) {
      const bc = new BroadcastChannel("dim");

      bc.postMessage("login");
      bc.close();
    }

    dispatch({
      type: AUTH_LOGIN_OK,
      payload,
    });
  } catch (err) {
    if (err.name === "TypeError") {
      dispatch({
        type: AUTH_LOGIN_ERR,
        payload: "Network Error",
      });
    } else {
      dispatch({
        type: AUTH_LOGIN_ERR,
        payload: err,
      });
    }
  }
};

export const logout = () => async (dispatch) => {
  document.cookie = "token=;expires=Thu, 01 Jan 1970 00:00:00 GMT";

  if ("BroadcastChannel" in window) {
    const bc = new BroadcastChannel("dim");

    bc.postMessage("logout");
    bc.close();
  }

  dispatch({ type: AUTH_LOGOUT });
};

export const updateAuthToken = (token) => (dispatch) => {
  dispatch({
    type: AUTH_UPDATE_TOKEN,
    payload: token,
  });
};

export const register = (username, password, invite) => async (dispatch) => {
  dispatch({ type: AUTH_REGISTER_START });

  const config = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      username: username,
      password: password,
      invite_token: invite,
    }),
  };

  try {
    const res = await fetch("/api/v1/auth/register", config);
    const payload = await res.json();

    if (res.status !== 200) {
      return dispatch({
        type: AUTH_REGISTER_ERR,
        payload: res.statusText,
      });
    } else if (!!payload.error) {
      return dispatch({
        type: AUTH_REGISTER_ERR,
        payload: payload.error,
      });
    }

    Promise.resolve(dispatch({ type: AUTH_REGISTER_OK }));
  } catch (err) {
    dispatch({
      type: AUTH_REGISTER_ERR,
      payload: err,
    });
  }
};

export const changePassword =
  (oldPassword, newPassword) => async (dispatch, getState) => {
    const token = getState().auth.token;

    const config = {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        authorization: token,
      },
      body: JSON.stringify({
        old_password: oldPassword,
        new_password: newPassword,
      }),
    };

    try {
      const res = await fetch("/api/v1/user/password", config);

      if (res.status !== 200) {
        dispatch(
          addNotification({
            msg: "Failed to change password.",
          })
        );

        return;
      }

      dispatch(
        addNotification({
          msg: "Your password has now been updated.",
        })
      );
    } catch (err) {
      dispatch(
        addNotification({
          msg: "Failed to change password.",
        })
      );
    }
  };

export const delAccount = (password) => async (dispatch, getState) => {
  dispatch({ type: DEL_ACCOUNT_START });

  const token = getState().auth.token;

  const config = {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      Authorization: token,
    },
    body: JSON.stringify({
      password: password,
    }),
  };

  try {
    const res = await fetch("/api/v1/user/delete", config);

    if (res.status !== 200) {
      dispatch({
        type: DEL_ACCOUNT_ERR,
        payload: res.statusText,
      });

      return;
    }

    dispatch({ type: DEL_ACCOUNT_OK });

    dispatch(
      addNotification({
        msg: "Your account has been deleted, you have been logged out.",
      })
    );
  } catch (err) {
    dispatch({
      type: DEL_ACCOUNT_ERR,
      payload: err,
    });
  }
};

export const checkAdminExists = () => async (dispatch) => {
  try {
    const res = await fetch("/api/v1/auth/admin_exists");

    if (res.status !== 200) {
      return dispatch({
        type: AUTH_CHECK_ADMIN_ERR,
        payload: res.statusText,
      });
    }

    const payload = await res.json();

    dispatch({
      type: AUTH_CHECK_ADMIN_OK,
      payload,
    });
  } catch (err) {
    dispatch({
      type: AUTH_CHECK_ADMIN_ERR,
      payload: err,
    });
  }
};

export const createNewInvite = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: CREATE_NEW_INVITE_START });

  const config = {
    method: "POST",
    headers: {
      authorization: token,
    },
  };

  try {
    const res = await fetch("/api/v1/auth/new_invite", config);

    if (res.status !== 200) {
      return dispatch({
        type: CREATE_NEW_INVITE_ERR,
        payload: res.statusText,
      });
    }

    const payload = await res.json();

    dispatch({
      type: CREATE_NEW_INVITE_OK,
      payload,
    });

    dispatch(
      addNotification({
        msg: `Successfuly created a new invite token: ${payload.token}.`,
      })
    );
  } catch (err) {
    dispatch({
      type: CREATE_NEW_INVITE_ERR,
      payload: err,
    });
  }
};

export const delInvite = (inviteToken) => async (dispatch, getState) => {
  const token = getState().auth.token;

  const config = {
    method: "DELETE",
    headers: {
      authorization: token,
    },
  };

  try {
    const res = await fetch(`/api/v1/auth/token/${inviteToken}`, config);

    if (res.status !== 200) {
      dispatch(
        addNotification({
          msg: `Could not delete invite token: ${inviteToken}`,
        })
      );

      return;
    }

    dispatch(
      addNotification({
        msg: `Successfuly deleted invite token: ${inviteToken}`,
      })
    );
  } catch (err) {
    dispatch({
      type: CREATE_NEW_INVITE_ERR,
      payload: err,
    });
  }
};

export const fetchInvites = () => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_INVITES_START });

  try {
    const config = {
      headers: {
        authorization: token,
      },
    };

    const res = await fetch("/api/v1/auth/invites", config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_INVITES_ERR,
        payload: res.statusText,
      });
    }

    const payload = await res.json();

    dispatch({
      type: FETCH_INVITES_OK,
      payload,
    });
  } catch (err) {
    dispatch({
      type: FETCH_INVITES_ERR,
      payload: err,
    });
  }
};
