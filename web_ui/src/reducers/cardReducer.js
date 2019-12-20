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

const fetch_cards = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const fetch_media_info = {
    info: {},
    fetching: false,
    fetched: false,
    error: null
};

const fetch_extra_media_info = {
    info: {},
    fetching: false,
    fetched: false,
    error: null
};

const fetch_media_seasons = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const fetch_media_season_episodes = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const initialState = {
    fetch_cards,
    fetch_media_info,
    fetch_extra_media_info,
    fetch_media_seasons,
    fetch_media_season_episodes
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_CARDS_START:
            return {
                ...state,
                fetch_cards: {
                    ...fetch_cards,
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_CARDS_OK:
            return {
                ...state,
                fetch_cards: {
                    ...fetch_cards,
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_CARDS_ERR:
            return {
                ...state,
                fetch_cards: {
                    ...fetch_cards,
                    items: [],
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_MEDIA_INFO_START:
            return {
                ...state,
                fetch_media_info: {
                    ...fetch_media_info,
                    info: {},
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_MEDIA_INFO_OK:
            return {
                ...state,
                fetch_media_info: {
                    ...fetch_media_info,
                    info: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_MEDIA_INFO_ERR:
            return {
                ...state,
                fetch_media_info: {
                    ...fetch_media_info,
                    info: {},
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_EXTRA_MEDIA_INFO_START:
            return {
                ...state,
                fetch_extra_media_info: {
                    ...fetch_extra_media_info,
                    info: {},
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_EXTRA_MEDIA_INFO_OK:
            return {
                ...state,
                fetch_extra_media_info: {
                    ...fetch_extra_media_info,
                    info: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_EXTRA_MEDIA_INFO_ERR:
            return {
                ...state,
                fetch_extra_media_info: {
                    ...fetch_extra_media_info,
                    info: {},
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_MEDIA_SEASONS_START:
            return {
                ...state,
                fetch_media_seasons: {
                    ...fetch_media_seasons,
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASONS_OK:
            return {
                ...state,
                fetch_media_seasons: {
                    ...fetch_media_seasons,
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASONS_ERR:
            return {
                ...state,
                fetch_media_seasons: {
                    ...fetch_media_seasons,
                    items: [],
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_MEDIA_SEASON_EPISODES_START:
            return {
                ...state,
                fetch_media_season_episodes: {
                    ...fetch_media_season_episodes,
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASON_EPISODES_OK:
            return {
                ...state,
                fetch_media_season_episodes: {
                    ...fetch_media_seasons,
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_MEDIA_SEASON_EPISODES_ERR:
            return {
                ...state,
                fetch_media_season_episodes: {
                    ...fetch_media_season_episodes,
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