import { SET_TRACKS, UPDATE_TRACK } from "../../actions/types";

export default function trackReducer(state, action) {
  switch (action.type) {
    case SET_TRACKS:
      return {
        video: {
          ...state.video,
          list: action.tracks.video,
        },
        audio: {
          ...state.audio,
          list: action.tracks.audio,
        },
        subtitle: {
          ...state.subtitle,
          list: action.tracks.subtitle,
        },
      };
    case UPDATE_TRACK:
      return {
        ...state,
        [action.track]: {
          ...state[action.track],
          ...action.data,
        },
      };
    default:
      return state;
  }
}
