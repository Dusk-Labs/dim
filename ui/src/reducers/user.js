import {
  FETCH_USER_START,
  FETCH_USER_OK,
  FETCH_USER_ERR,
  CHANGE_USERNAME_ERR,
  CHANGE_USERNAME_START,
  CHANGE_USERNAME_OK,
  CHANGE_AVATAR_START,
  CHANGE_AVATAR_OK,
  CHANGE_AVATAR_ERR,
} from "../actions/types";

const changeUsername = {
  changing: false,
  changed: true,
  error: null,
};

const changeAvatar = {
  changing: false,
  changed: true,
  error: null,
};

const initialState = {
  info: {},
  fetching: false,
  fetched: false,
  error: null,
  changeUsername,
  changeAvatar,
};

export default function userReducer(state = initialState, action) {
  switch (action.type) {
    case FETCH_USER_START:
      return {
        ...state,
        info: {},
        fetching: true,
        fetched: false,
        error: null,
        changeUsername: {
          changing: false,
          changed: false,
          error: null,
        },
        changeAvatar: {
          changing: false,
          changed: false,
          error: null,
        },
      };
    case FETCH_USER_OK:
      return {
        ...state,
        fetching: false,
        fetched: true,
        info: action.payload,
      };
    case FETCH_USER_ERR:
      return {
        ...state,
        fetching: false,
        fetched: true,
        error: action.payload,
      };
    case CHANGE_USERNAME_START:
      return {
        ...state,
        changeUsername: {
          changing: true,
          changed: false,
          error: null,
        },
      };
    case CHANGE_USERNAME_OK:
      return {
        ...state,
        changeUsername: {
          ...state.changeUsername,
          changing: false,
          changed: true,
        },
      };
    case CHANGE_USERNAME_ERR:
      return {
        ...state,
        changeUsername: {
          ...state.changeUsername,
          changing: false,
          err: action.payload,
        },
      };
    case CHANGE_AVATAR_START:
      return {
        ...state,
        changeAvatar: {
          changing: true,
          changed: false,
          error: null,
        },
      };
    case CHANGE_AVATAR_OK:
      return {
        ...state,
        changeAvatar: {
          ...state.changeAvatar,
          changing: false,
          changed: true,
        },
      };
    case CHANGE_AVATAR_ERR:
      return {
        ...state,
        changeAvatar: {
          ...state.changeAvatar,
          changing: false,
          err: action.payload,
        },
      };
    default:
      return state;
  }
}
