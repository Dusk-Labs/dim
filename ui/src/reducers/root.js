import { combineReducers } from "redux";

import auth from "./auth";
import ws from "./ws.js";
import user from "./user.js";
import library from "./library.js";
import fileBrowser from "./fileBrowser.js";
import search from "./search.js";
import card from "./card.js";
import banner from "./banner.js";
import settings from "./settings.js";
import video from "./video/index";

export default combineReducers({
  auth,
  ws,
  user,
  library,
  fileBrowser,
  search,
  card,
  banner,
  settings,
  video
});
