import {
  FETCH_DIRECTORIES_START,
  FETCH_DIRECTORIES_OK,
  FETCH_DIRECTORIES_ERR
} from "../actions/types";

const initialState = {
  items: [],
  fetching: false,
  fetched: false,
  error: null
};

export default function fileBrowserReducer(state = initialState, action) {
  switch(action.type) {
    case FETCH_DIRECTORIES_START:
      return {
        ...state,
        items: [],
        fetching: true,
        fetched: false,
        error: null
      };
    case FETCH_DIRECTORIES_OK:
      return {
        ...state,
        items: action.payload.dirs,
        fetching: false,
        fetched: true
      };
    case FETCH_DIRECTORIES_ERR:
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
