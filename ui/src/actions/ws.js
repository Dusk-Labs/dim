import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR,
  WS_SHOW_RECONNECT,
  NOTIFICATIONS_ADD
} from "./types.js";

export const wsConnect = () => async (dispatch, getState) => {
  dispatch({ type: WS_CONNECT_START });

  // NOTE(val): this is needed because webpack fails to proxy websocket traffic through.
  const base = process.env.NODE_ENV === "development" ? `${window.location.hostname}:8000` : window.location.host;
  const host = window.location.protocol === "https:" ? `wss://${base}/ws` : `ws://${base}/ws`;

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
