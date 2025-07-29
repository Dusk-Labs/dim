import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR,
  WS_SHOW_RECONNECT,
} from "./types";

import { addNotification } from "../slices/notifications";

import type { AppDispatch, RootState } from "../store";

interface OnNewSocket {
  (newSocket: WebSocket): void;
}

export const wsConnect =
  (onNewSocket: OnNewSocket) =>
  async (dispatch: AppDispatch, getState: () => RootState) => {
    dispatch({ type: WS_CONNECT_START });

    const webSocketUrl = (() => {
      const url = new URL("/ws", window.location.href);
      url.protocol = url.protocol.replace("http", "ws");
      // Create React App's proxy feature has not worked consistently. This makes WebSocket
      // connections directly to the server instead of proxying through Create React App.
      if (process.env.NODE_ENV === "development") {
        url.port = "8000";
      }
      return url.href;
    })();

    try {
      const ws = await new Promise<WebSocket>((resolve, reject) => {
        const socket = new WebSocket(webSocketUrl);

        socket.onopen = () => resolve(socket);
        socket.onerror = (e) => reject(e);
      });

      const { showReconnect } = getState().ws;

      if (showReconnect) {
        dispatch(
          addNotification({
            msg: "Connection to the server has been restored.",
          })
        );
      }

      onNewSocket(ws);

      dispatch({
        type: WS_CONNECTED,
      });
    } catch (err) {
      dispatch({
        type: WS_CONNECT_ERR,
        payload: err instanceof Object ? err.toString() : "Unknown error",
      });
    }
  };

export const wsShowReconnect = () => async (dispatch: AppDispatch) => {
  dispatch({
    type: WS_SHOW_RECONNECT,
  });
};
