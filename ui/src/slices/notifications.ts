import { PayloadAction, createSlice } from "@reduxjs/toolkit";

/*
 * A message displayed to the user in a toast window.
 */
export type Notification = {
  msg: string;
};

/*
 * The slice's state. Contains a list of notifications to display.
 */
type NotificationState = {
  list: Array<Notification>;
};

const initialState: NotificationState = {
  list: [],
};

export const notifications = createSlice({
  name: "notifications",
  initialState,
  reducers: {
    addNotification: (state, action: PayloadAction<Notification>) => {
      state.list.push(action.payload);
    },
    removeNotification: (state, action: PayloadAction<number>) => {
      state.list.splice(action.payload, 1);
    },
    clearNotifications: (state) => {
      state.list.splice(0, state.list.length);
    },
  },
});

export const { addNotification, removeNotification, clearNotifications } =
  notifications.actions;

export default notifications.reducer;
