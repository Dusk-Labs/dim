import {
  FETCH_USER_SETTINGS,
  FETCH_USER_SETTINGS_START,
  FETCH_USER_SETTINGS_ERR,
  FETCH_GLOBAL_SETTINGS_START,
  FETCH_GLOBAL_SETTINGS,
  FETCH_GLOBAL_SETTINGS_ERR
} from "../actions/types.js";

const initialState = {
  globalSettings: {},
  userSettings: {},
  fetching: false,
  fetched: false,
  error: null
};

export default function settingsReducer(state = initialState, action) {
  switch(action.type) {
    case FETCH_USER_SETTINGS_START:
      return {
        ...state,
        fetching: true,
        fetched: false,
        error: null
      };
    case FETCH_USER_SETTINGS_ERR:
      return {
        ...state,
        fetching: false,
        fetched: true,
        error: action.payload
      };
    case FETCH_USER_SETTINGS:
      return {
        ...state,
        fetching: false,
        fetched: true,
        error: null,
        userSettings: action.payload
      };
    case FETCH_GLOBAL_SETTINGS_START:
      return {
        ...state,
        fetching: true,
        fetched: false,
        error: null
      };
    case FETCH_GLOBAL_SETTINGS_ERR:
      return {
        ...state,
        fetching: false,
        fetched: true,
        error: action.payload
      };
    case FETCH_GLOBAL_SETTINGS:
      return {
        ...state,
        fetching: false,
        fetched: true,
        globalSettings: action.payload
      };
    default:
      return state;
  }
}
