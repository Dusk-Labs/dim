import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR,
  WS_SHOW_RECONNECT
} from "../actions/types";

const initialState = {
  conn: null,
  connecting: false,
  connected: false,
  error: null,
  showReconnect: false
};

export default function wsReducer(state = initialState, action) {
  switch(action.type) {
    case WS_CONNECT_START:
      return {
        ...state,
        conn: null,
        connecting: true,
        connected: false,
        error: null
      };
    case WS_CONNECTED:
      return {
        ...state,
        connecting: false,
        connected: true,
        conn: action.conn,
        showReconnect: false
      };
    case WS_CONNECT_ERR:
      return {
        ...state,
        connecting: false,
        connected: false,
        error: action.payload
      };
    case WS_SHOW_RECONNECT:
      return {
        ...state,
        showReconnect: true
      };
    default:
      return state;
  }
}
