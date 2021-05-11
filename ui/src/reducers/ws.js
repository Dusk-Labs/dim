import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR
} from "../actions/types.js";

const initialState = {
  conn: null,
  connecting: false,
  connected: false,
  error: null
};

export default function wsReducer(state = initialState, action) {
  switch(action.type) {
    case WS_CONNECT_START:
      return {
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
        conn: action.conn
      };
    case WS_CONNECT_ERR:
      return {
        ...state,
        connecting: false,
        connected: false,
        error: action.payload
      };
    default:
      return state;
  }
}
