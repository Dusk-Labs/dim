import { combineReducers } from "redux";

import auth from "./auth";
import user from "./user.js";
import library from "./library.js";
import fileBrowser from "./fileBrowser.js";
import search from "./search.js";
import card from "./card.js";
import banner from "./banner.js";

export default combineReducers({
  auth,
  user,
  library,
  fileBrowser,
  search,
  card,
  banner
});
