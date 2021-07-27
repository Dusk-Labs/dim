import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR,
  WS_SHOW_RECONNECT,
  NOTIFICATIONS_ADD
} from "./types.js";

export const wsConnect = () => async (dispatch, getState) => {
  dispatch({ type: WS_CONNECT_START });

  const host = `ws://${window.location.hostname}:3012/`;

  try {
    const ws = await new Promise((resolve, reject) => {
      const socket = new WebSocket(host);

      socket.onopen = () => resolve(socket);
      socket.onerror = (e) => reject(e);
    });

    const { showReconnect } = getState().ws;

    if (showReconnect) {
      dispatch({
        type: NOTIFICATIONS_ADD,
        payload: {
          msg: "Connection to the server has been restored."
        }
      });
    }

    dispatch({
      type: WS_CONNECTED,
      conn: ws
    });
  } catch(err) {
    dispatch({
      type: WS_CONNECT_ERR,
      payload: err
    });
  }
};

export const wsShowReconnect = () => async (dispatch) => {
  dispatch({
    type: WS_SHOW_RECONNECT
  });
};
