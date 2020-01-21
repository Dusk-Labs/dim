import { combineReducers } from "redux";

import libraryReducer from "./libraryReducer.js";
import userReducer from "./userReducer.js";
import bannerReducer from "./bannerReducer.js";
import fileBrowserReducer from "./fileBrowserReducer.js";
import searchReducer from "./searchReducer.js";
import cardReducer from "./cardReducer.js";
import authReducer from "./authReducer";

export default combineReducers({
    authReducer,
    userReducer,
    libraryReducer,
    fileBrowserReducer,
    searchReducer,
    cardReducer,
    bannerReducer,
});
