import { combineReducers } from "redux";

import libraryReducer from "./libraryReducer.js";
import hostReducer from "./hostReducer.js";
import userReducer from "./userReducer.js";
import bannerReducer from "./bannerReducer.js";
import fileBrowserReducer from "./fileBrowserReducer.js";
import searchReducer from "./searchReducer.js";
import cardReducer from "./cardReducer.js";
import videoPlayerReducer from "./videoPlayerReducer.js";

export default combineReducers({
    userReducer,
    hostReducer,
    libraryReducer,
    fileBrowserReducer,
    searchReducer,
    cardReducer,
    bannerReducer,
    videoPlayerReducer
});