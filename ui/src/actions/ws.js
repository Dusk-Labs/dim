import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR
} from "./types.js";

export const wsConnect = () => async (dispatch) => {
  dispatch({ type: WS_CONNECT_START });

  const host = `ws://${window.location.hostname}:3012/`;

  try {
    const ws = await new Promise((resolve, reject) => {
      const socket = new WebSocket(host);

      socket.onopen = () => resolve(socket);
      socket.onerror = (e) => reject(e);
    });

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
