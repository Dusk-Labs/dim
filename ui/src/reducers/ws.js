import {
  WS_CONNECT_START,
  WS_CONNECTED,
  WS_CONNECT_ERR
} from "../actions/types.js";

const initialState = {
  ws: null,
  connecting: false,
  connected: false,
  error: null
};

export default function wsReducer(state = initialState, action) {
  switch(action.type) {
    case WS_CONNECT_START:
      return {
        ws: null,
        connecting: true,
        connected: false,
        error: null
      };
    case WS_CONNECTED:
      return {
        ...state,
        connecting: false,
        connected: true,
        ws: action.ws
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
