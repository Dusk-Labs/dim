import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query/react";

import auth from "./reducers/auth";
import library from "./reducers/library";
import notifications from "./slices/notifications";
import settings from "./reducers/settings";
import user from "./reducers/user";
import video from "./reducers/video/index";
import ws from "./reducers/ws";
import v1 from "./api/v1";

export const store = configureStore({
  reducer: {
    auth,
    ws,
    user,
    library,
    video,
    settings,
    notifications,
    [v1.reducerPath]: v1.reducer,
  },
  middleware: (getDefaultMiddleware) => {
    return getDefaultMiddleware().concat(v1.middleware);
  },
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
