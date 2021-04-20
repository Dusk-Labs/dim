import {
  FETCH_CARDS_START,
  FETCH_CARDS_OK,
  FETCH_CARDS_ERR,
  FETCH_MEDIA_INFO_START,
  FETCH_MEDIA_INFO_OK,
  FETCH_MEDIA_INFO_ERR,
  FETCH_MEDIA_INFO_CLEAR,
  FETCH_EXTRA_MEDIA_INFO_START,
  FETCH_EXTRA_MEDIA_INFO_OK,
  FETCH_EXTRA_MEDIA_INFO_ERR,
  FETCH_MEDIA_SEASONS_START,
  FETCH_MEDIA_SEASONS_OK,
  FETCH_MEDIA_SEASONS_ERR,
  FETCH_MEDIA_SEASON_EPISODES_START,
  FETCH_MEDIA_SEASON_EPISODES_OK,
  FETCH_MEDIA_SEASON_EPISODES_ERR
} from "./types.js";

export const fetchCards = (path) => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_CARDS_START });

  try {
    const config = {
      headers: {
        "authorization": token,
      }
    };

    const res = await fetch(path, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_CARDS_ERR,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    dispatch({
      type: FETCH_CARDS_OK,
      payload
    });
  } catch(err) {
    dispatch({
      type: FETCH_CARDS_ERR,
      payload: err
    });
  }
};

export const fetchMediaInfo = (id) => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_MEDIA_INFO_START });

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`//${window.host}:8000/api/v1/media/${id}`, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_MEDIA_INFO_ERR,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    dispatch({
      type: FETCH_MEDIA_INFO_OK,
      payload
    });
  } catch(err) {
    dispatch({
      type: FETCH_MEDIA_INFO_ERR,
      payload: err
    });
  }
};

export const clearMediaInfo = () => async (dispatch) => {
  dispatch({ type: FETCH_MEDIA_INFO_CLEAR });
};

export const fetchExtraMediaInfo = (id) => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_EXTRA_MEDIA_INFO_START });

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`//${window.host}:8000/api/v1/media/${id}/info`, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_EXTRA_MEDIA_INFO_ERR,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    if (payload.error) {
      return dispatch({
        type: FETCH_EXTRA_MEDIA_INFO_ERR,
        payload: payload.error
      });
    }

    dispatch({
      type: FETCH_EXTRA_MEDIA_INFO_OK,
      payload
    });
  } catch(err) {
    dispatch({
      type: FETCH_EXTRA_MEDIA_INFO_ERR,
      payload: err
    });
  }
};

export const fetchMediaSeasons = (id) => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_MEDIA_SEASONS_START });

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`//${window.host}:8000/api/v1/tv/${id}/season`, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_MEDIA_SEASONS_ERR,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    dispatch({
      type: FETCH_MEDIA_SEASONS_OK,
      payload
    });
  } catch(err) {
    dispatch({
      type: FETCH_MEDIA_SEASONS_ERR,
      payload: err
    });
  }
};

export const fetchMediaSeasonEpisodes = (id, season) => async (dispatch, getState) => {
  const token = getState().auth.token;

  dispatch({ type: FETCH_MEDIA_SEASON_EPISODES_START });

  try {
    const config = {
      headers: {
        "authorization": token
      }
    };

    const res = await fetch(`//${window.host}:8000/api/v1/tv/${id}/season/${season}/episode`, config);

    if (res.status !== 200) {
      return dispatch({
        type: FETCH_MEDIA_SEASON_EPISODES_ERR,
        payload: res.statusText
      });
    }

    const payload = await res.json();

    dispatch({
      type: FETCH_MEDIA_SEASON_EPISODES_OK,
      payload
    });
  } catch(err) {
    dispatch({
      type: FETCH_MEDIA_SEASON_EPISODES_ERR,
      payload: err
    });
  }
};
