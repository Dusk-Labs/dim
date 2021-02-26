import {
    FETCH_CARDS_START,
    FETCH_CARDS_OK,
    FETCH_CARDS_ERR,
    FETCH_MEDIA_INFO_START,
    FETCH_MEDIA_INFO_OK,
    FETCH_MEDIA_INFO_ERR,
    FETCH_EXTRA_MEDIA_INFO_START,
    FETCH_EXTRA_MEDIA_INFO_OK,
    FETCH_EXTRA_MEDIA_INFO_ERR,
    FETCH_MEDIA_SEASONS_START,
    FETCH_MEDIA_SEASONS_OK,
    FETCH_MEDIA_SEASONS_ERR,
    FETCH_MEDIA_SEASON_EPISODES_START,
    FETCH_MEDIA_SEASON_EPISODES_OK,
    FETCH_MEDIA_SEASON_EPISODES_ERR,
} from "../actions/types.js";

const cards = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const media_info = {
    info: {},
    fetching: false,
    fetched: false,
    error: null
};

const extra_media_info = {
    info: {},
    fetching: false,
    fetched: false,
    error: null
};

const media_seasons = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const media_season_episodes = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const initialState = {
    cards,
    media_info,
    extra_media_info,
    media_seasons,
    media_season_episodes
};

export default function cardReducer(state = initialState, action) {
    switch(action.type) {
        case FETCH_CARDS_START:
            return {
                ...state,
                cards: {
                    ...cards,
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_CARDS_OK:
            return {
                ...state,
                cards: {
                    ...cards,
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_CARDS_ERR:
            return {
                ...state,
                cards: {
                    ...cards,
                    items: [],
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_MEDIA_INFO_START:
            return {
                ...state,
                media_info: {
                    ...media_info,
                    info: {},
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_MEDIA_INFO_OK:
            return {
                ...state,
                media_info: {
                    ...media_info,
                    info: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_MEDIA_INFO_ERR:
            return {
                ...state,
                media_info: {
                    ...media_info,
                    info: {},
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_EXTRA_MEDIA_INFO_START:
            return {
                ...state,
                extra_media_info: {
                    ...extra_media_info,
                    info: {},
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_EXTRA_MEDIA_INFO_OK:
            return {
                ...state,
                extra_media_info: {
                    ...extra_media_info,
                    info: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_EXTRA_MEDIA_INFO_ERR:
            return {
                ...state,
                extra_media_info: {
                    ...extra_media_info,
                    info: {},
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_MEDIA_SEASONS_START:
            return {
                ...state,
                media_seasons: {
                    ...media_seasons,
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASONS_OK:
            return {
                ...state,
                media_seasons: {
                    ...media_seasons,
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASONS_ERR:
            return {
                ...state,
                media_seasons: {
                    ...media_seasons,
                    items: [],
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_MEDIA_SEASON_EPISODES_START:
            return {
                ...state,
                media_season_episodes: {
                    ...media_season_episodes,
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASON_EPISODES_OK:
            return {
                ...state,
                media_season_episodes: {
                    ...media_seasons,
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASON_EPISODES_ERR:
            return {
                ...state,
                media_season_episodes: {
                    ...media_season_episodes,
                    items: [],
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        default:
            return state;
    }
}