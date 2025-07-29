import {
  SET_GID,
  SET_MANIFEST_STATE,
  SET_TRACKS,
  SET_SHOW_SUB_SWITCHER,
  UPDATE_TRACK,
  UPDATE_VIDEO,
  CLEAR_VIDEO_DATA,
  SET_SHOW_SETTINGS,
} from "./types";

export const setGID = (gid) => async (dispatch) => {
  dispatch({
    type: SET_GID,
    gid,
  });
};

export const setManifestState = (state) => async (dispatch) => {
  dispatch({
    type: SET_MANIFEST_STATE,
    state,
  });
};

export const setTracks = (tracks) => async (dispatch) => {
  dispatch({
    type: SET_TRACKS,
    tracks,
  });
};

export const updateTrack = (track, data) => async (dispatch) => {
  dispatch({
    type: UPDATE_TRACK,
    track,
    data,
  });
};

export const updateVideo = (data) => async (dispatch) => {
  dispatch({
    type: UPDATE_VIDEO,
    data,
  });
};

export const toggleShowSubSwitcher = () => async (dispatch, getState) => {
  const state = getState().video.showSubSwitcher;

  dispatch({
    type: SET_SHOW_SUB_SWITCHER,
    state: !state,
  });
};

export const toggleShowSettings = () => async (dispatch, getState) => {
  const state = getState().video.showSettings;

  dispatch({
    type: SET_SHOW_SETTINGS,
    state: !state,
  });
};

export const incIdleCount = () => async (dispatch, getState) => {
  const state = getState().video.idleCount;

  dispatch({
    type: UPDATE_VIDEO,
    data: {
      idleCount: state + 1,
    },
  });
};

export const clearVideoData = () => async (dispatch) => {
  dispatch({ type: CLEAR_VIDEO_DATA });
};
