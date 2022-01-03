import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query/react";

import auth from "./reducers/auth";
import banner from "./reducers/banner";
import card from "./reducers/card";
import fileBrowser from "./reducers/fileBrowser";
import library from "./reducers/library";
import media from "./reducers/media/index";
import notifications from "./reducers/notifications";
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
    fileBrowser,
    card,
    banner,
    video,
    settings,
    notifications,
    media,
    [v1.reducerPath]: v1.reducer
  },
  middleware: (getDefaultMiddleware) => {
    return getDefaultMiddleware().concat(v1.middleware);
  }
});

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
