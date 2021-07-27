import {
  NOTIFICATIONS_ADD,
  NOTIFICATIONS_REMOVE,
  NOTIFICATIONS_CLEAR
} from "./types.js";

export const notificationsAdd = (props) => async (dispatch) => {
  dispatch({
    type: NOTIFICATIONS_ADD,
    payload: props
  });
};

export const notificationsRemove = (props) => async (dispatch) => {
  dispatch({
    type: NOTIFICATIONS_REMOVE,
    payload: props
  });
};

export const notificationsClear = () => async (dispatch) => {
  dispatch({type: NOTIFICATIONS_CLEAR});
};
