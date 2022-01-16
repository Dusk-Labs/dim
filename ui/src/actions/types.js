/*
 * == USER ACTIONS ==
 */

export const FETCH_USER_START = "FETCH_USER_START";
export const FETCH_USER_OK = "FETCH_USER_OK";
export const FETCH_USER_ERR = "FETCH_USER_ERR";

export const CHANGE_USERNAME_START = "CHANGE_USERNAME_START";
export const CHANGE_USERNAME_OK = "CHANGE_USERNAME_OK";
export const CHANGE_USERNAME_ERR = "CHANGE_USERNAME_ERR";

export const CHANGE_AVATAR_START = "CHANGE_AVATAR_START";
export const CHANGE_AVATAR_OK = "CHANGE_AVATAR_OK";
export const CHANGE_AVATAR_ERR = "CHANGE_AVATAR_ERR";

/*
 * == LIBRARY ACTIONS ==
 */

export const FETCH_LIBRARIES_START = "FETCH_LIBRARIES_START";
export const FETCH_LIBRARIES_OK = "FETCH_LIBRARIES_OK";
export const FETCH_LIBRARIES_ERR = "FETCH_LIBRARIES_ERR";

export const FETCH_LIBRARY_UNMATCHED_START = "FETCH_LIBRARY_UNMATCHED_START";
export const FETCH_LIBRARY_UNMATCHED_OK = "FETCH_LIBRARY_UNMATCHED_OK";
export const FETCH_LIBRARY_UNMATCHED_ERR = "FETCH_LIBRARY_UNMATCHED_ERR";

export const FETCH_LIBRARY_INFO = "FETCH_LIBRARY_INFO";
export const FETCH_LIBRARY_MEDIA = "FETCH_LIBRARY_MEDIA";

export const NEW_LIBRARY_START = "NEW_LIBRARY_START";
export const NEW_LIBRARY_OK = "NEW_LIBRARY_OK";
export const NEW_LIBRARY_ERR = "NEW_LIBRARY_ERR";

export const DEL_LIBRARY_START = "DEL_LIBRARY_START";
export const DEL_LIBRARY_OK = "DEL_LIBRARY_OK";
export const DEL_LIBRARY_ERR = "DEL_LIBRARY_ERR";

export const RM_LIBRARY = "RM_LIBRARY";
export const ADD_LIBRARY = "ADD_LIBRARY";
export const SCAN_START = "SCAN_START";
export const SCAN_STOP = "SCAN_STOP";

/*
 * == VIDEO PLAYER ACTIONS ==
 */

export const TRANSCODE_START = "TRANSCODE_START";
export const TRANSCODE_OK = "TRANSCODE_OK";
export const TRANSCODE_ERR = "TRANSCODE_ERR";

export const DEL_TRANSCODE_START = "DEL_TRANSCODE_START";
export const DEL_TRANSCODE_OK = "DEL_TRANSCODE_OK";
export const DEL_TRANSCODE_ERR = "DEL_TRANSCODE_ERR";

/*
 * AUTH ACTIONS
 */

export const AUTH_LOGIN_START = "AUTH_LOGIN_START";
export const AUTH_LOGIN_OK = "AUTH_LOGIN_OK";
export const AUTH_LOGIN_ERR = "AUTH_LOGIN_ERR";

export const AUTH_UPDATE_TOKEN = "AUTH_UPDATE_TOKEN";
export const AUTH_LOGOUT = "AUTH_LOGOUT";

export const AUTH_REGISTER_START = "AUTH_REGISTER_START";
export const AUTH_REGISTER_ERR = "AUTH_REGISTER_ERR";
export const AUTH_REGISTER_OK = "AUTH_REGISTER_OK";

export const AUTH_CHECK_ADMIN_OK = "AUTH_CHECK_ADMIN_OK";
export const AUTH_CHECK_ADMIN_ERR = "AUTH_CHECK_ADMIN_ERR";

export const CREATE_NEW_INVITE_START = "CREATE_NEW_INVITE_START";
export const CREATE_NEW_INVITE_OK = "CREATE_NEW_INVITE_OK";
export const CREATE_NEW_INVITE_ERR = "CREATE_NEW_INVITE_ERR";

export const FETCH_INVITES_START = "FETCH_INVITES_START";
export const FETCH_INVITES_OK = "FETCH_INVITES_OK";
export const FETCH_INVITES_ERR = "FETCH_INVITES_ERR";

export const DEL_ACCOUNT_START = "DEL_ACCOUNT_START";
export const DEL_ACCOUNT_OK = "DEL_ACCOUNT_OK";
export const DEL_ACCOUNT_ERR = "DEL_ACCOUNT_ERR";

/*
 * WS ACTIONS
 */

export const WS_CONNECT_START = "WS_CONNECT_START";
export const WS_CONNECTED = "WS_CONNECTED";
export const WS_CONNECT_ERR = "WS_CONNECT_ERR";
export const WS_SHOW_RECONNECT = "WS_SHOW_RECONNECT";

/*
 * SETTINGS ACTIONS
 */

export const FETCH_USER_SETTINGS_START = "FETCH_USER_SETTINGS_START";
export const FETCH_USER_SETTINGS_OK = "FETCH_USER_SETTINGS_OK";
export const FETCH_USER_SETTINGS_ERR = "FETCH_USER_SETTINGS_ERR";
export const UPDATE_USER_SETTINGS = "UPDATE_USER_SETTINGS";

export const FETCH_GLOBAL_SETTINGS_START = "FETCH_GLOBAL_SETTINGS_START";
export const FETCH_GLOBAL_SETTINGS_OK = "FETCH_GLOBAL_SETTINGS_OK";
export const FETCH_GLOBAL_SETTINGS_ERR = "FETCH_GLOBAL_SETTINGS_START";
export const UPDATE_GLOBAL_SETTINGS = "UPDATE_GLOBAL_SETTINGS";

/*
 * VIDEO ACTIONS
 */

export const SET_GID = "SET_GID";
export const SET_MANIFEST_STATE = "SET_MANIFEST_STATE";
export const SET_TRACKS = "SET_TRACKS";
export const UPDATE_TRACK = "UPDATE_TRACK";
export const UPDATE_VIDEO = "UPDATE_VIDEO";
export const SET_SHOW_SUB_SWITCHER = "SET_SHOW_SUB_SWITCHER";
export const SET_SHOW_SETTINGS = "SET_SHOW_SETTINGS";
export const CLEAR_VIDEO_DATA = "CLEAR_VIDEO_DATA";
