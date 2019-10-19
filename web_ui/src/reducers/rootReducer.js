import { combineReducers } from "redux";

import libraryReducer from "./libraryReducer.js";
import hostReducer from "./hostReducer.js";
import userReducer from "./userReducer.js";
import dashboardReducer from "./dashboardReducer.js";
import bannerReducer from "./bannerReducer.js";
import fileBrowserReducer from "./fileBrowserReducer.js";

export default combineReducers({
    user: userReducer,
    hosts: hostReducer,
    libraryReducer,
    fileBrowserReducer,
    dashboard: dashboardReducer,
    banners: bannerReducer
});