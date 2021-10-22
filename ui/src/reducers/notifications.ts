import { AnyAction } from "redux";
import {
  NOTIFICATIONS_ADD,
  NOTIFICATIONS_REMOVE,
  NOTIFICATIONS_CLEAR
} from "../actions/types.js";

export type Notification = {
  msg: string
}

type NotificationState = {
  list: Array<Notification>
}

const initialState: NotificationState = {
  list: []
};

export default function notificationsReducer(state = initialState, action: AnyAction) {
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
