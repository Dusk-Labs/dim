import {
  FETCH_MEDIA_INFO_OK,
  FETCH_MEDIA_INFO_ERR,
  FETCH_MEDIA_FILES_OK,
  FETCH_MEDIA_SEASONS_OK,
  FETCH_MEDIA_EPISODES_OK
} from "./types";

export const fetchMediaInfo = (id) => async (dispatch, getState) => {
  const token = getState().auth.token;

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`/api/v1/media/${id}`, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_MEDIA_INFO_ERR,
        id,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    dispatch({
      type: FETCH_MEDIA_INFO_OK,
      id,
      payload
    });
  } catch(err) {
    dispatch({
      type: FETCH_MEDIA_INFO_ERR,
      payload: err
    });
  }
};

export const fetchMediaFiles = (id) => async (dispatch, getState) => {
  const token = getState().auth.token;

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`/api/v1/media/${id}/files`, config);

    if (res.status !== 200) return;

    const payload = await res.json();

    dispatch({
      type: FETCH_MEDIA_FILES_OK,
      id,
      payload: payload
    });
  } catch(err) {}
};

export const fetchMediaSeasons = (id) => async (dispatch, getState) => {
  const token = getState().auth.token;

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`/api/v1/tv/${id}/season`, config);

    if (res.status !== 200) return;

    const payload = await res.json();

    const seasons = Object.values(payload).sort((a, b) => {
      return a.season_number - b.season_number;
    });

    dispatch({
      type: FETCH_MEDIA_SEASONS_OK,
      id,
      payload: seasons
    });
  } catch(err) {}
};

export const fetchMediaEpisodes = (id, seasonID) => async (dispatch, getState) => {
  const token = getState().auth.token;

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`/api/v1/season/${seasonID}/episodes`, config);

    if (res.status !== 200) return;

    const payload = await res.json();

    const episodes = payload.sort((a, b) => {
      return a.episode - b.episode;
    });

    dispatch({
      type: FETCH_MEDIA_EPISODES_OK,
      id,
      payload: episodes
    });
  } catch(err) {}
};
