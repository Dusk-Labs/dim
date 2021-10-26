import {
  FETCH_MEDIA_INFO_OK,
  FETCH_MEDIA_INFO_ERR,
  FETCH_MEDIA_FILES_OK,
  FETCH_MEDIA_SEASONS_OK,
  FETCH_MEDIA_EPISODES_OK
} from "../../actions/types";
import mediaInfoReducer from "./infoReducer";

const files = {
  items: [],
  fetching: false,
  fetched: false,
  error: null
};

const initialState = {};

const infoActions = [
  FETCH_MEDIA_INFO_OK,
  FETCH_MEDIA_INFO_ERR
];

export default function mediaReducer(state = initialState, action) {
  if (infoActions.includes(action.type)) return {
    ...state,
    [action.id]: {
      ...state[action.id],
      info: mediaInfoReducer(action)
    }
  };

  if (action.type === FETCH_MEDIA_FILES_OK) return {
    ...state,
    [action.id]: {
      ...state[action.id],
      files: {
        ...files,
        items: action.payload
      }
    }
  };

  if (action.type === FETCH_MEDIA_SEASONS_OK) return {
    ...state,
    [action.id]: {
      ...state[action.id],
      seasons: action.payload
    }
  };

  if (action.type === FETCH_MEDIA_EPISODES_OK) return {
    ...state,
    [action.id]: {
      ...state[action.id],
      episodes: action.payload
    }
  };

  return state;
}
