import {
  FETCH_MEDIA_INFO_OK,
  FETCH_MEDIA_INFO_ERR
} from "../../actions/types";

const info = {
  data: {},
  fetching: false,
  fetched: false,
  error: null
};

export default function mediaInfoReducer(action) {
  if (action.type === FETCH_MEDIA_INFO_OK) return {
    data: action.payload,
    fetching: false,
    fetched: true
  };

  if (action.type === FETCH_MEDIA_INFO_ERR) return {
    ...info,
    fetching: false,
    fetched: true,
    error: action.payload
  };

  return info;
}
