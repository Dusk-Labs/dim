import {
  NOTIFICATIONS_ADD,
  NOTIFICATIONS_REMOVE,
  NOTIFICATIONS_CLEAR
} from "../actions/types.js";

const initialState = {
  list: []
};

export default function notificationsReducer(state = initialState, action) {
  switch(action.type) {
    case NOTIFICATIONS_ADD:
      return {
        list: [...state.list, action.payload]
      };
    case NOTIFICATIONS_REMOVE:
      const newList = state.list.filter((_, i) => i !== action.payload);

      return {
        list: newList
      };
    case NOTIFICATIONS_CLEAR:
      return {
        list: []
      };
    default:
      return state;
  }
}
