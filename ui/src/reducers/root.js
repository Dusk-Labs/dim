import { combineReducers } from "redux";

import auth from "./auth";
import ws from "./ws";
import user from "./user";
import library from "./library";
import fileBrowser from "./fileBrowser";
import search from "./search";
import card from "./card";
import banner from "./banner";
import settings from "./settings";
import video from "./video/index";
import notifications from "./notifications";
import media from "./media/index";

export default combineReducers({
  auth,
  ws,
  user,
  library,
  fileBrowser,
  search,
  card,
  banner,
  video,
  settings,
  notifications,
  media
});
