import {
  FETCH_USER_START,
  FETCH_USER_OK,
  FETCH_USER_ERR
} from "../actions/types.js";

const initialState = {
  info: {},
  fetching: false,
  fetched: false,
  error: null
};

export default function userReducer(state = initialState, action) {
  switch(action.type) {
    case FETCH_USER_START:
      return {
        info: {},
        fetching: true,
        fetched: false,
        error: null
      };
    case FETCH_USER_OK:
      return {
        ...state,
        fetching: false,
        fetched: true,
        info: action.payload
      };
    case FETCH_USER_ERR:
      return {
        ...state,
        fetching: false,
        fetched: true,
        error: action.payload
      };
    default:
      return state;
  }
}
