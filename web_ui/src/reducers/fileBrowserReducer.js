import {
    FETCH_DIRECTORIES_START,
    FETCH_DIRECTORIES_OK,
    FETCH_DIRECTORIES_ERR
} from "../actions/types.js";

const initialState = {
    items: [],
    cache: {},
    fetching: false,
    fetched: false,
    error: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_DIRECTORIES_START:
            return {
                ...state,
                items: [],
                fetching: true,
                fetched: false,
                error: null
            }
        case FETCH_DIRECTORIES_OK:
            const items = action.payload.dirs.slice(1);

            return {
                ...state,
                items,
                cache: {
                    ...state.cache,
                    [action.payload.path]: items
                },
                fetching: false,
                fetched: true
            }
        case FETCH_DIRECTORIES_ERR:
            return {
                ...state,
                fetching: false,
                fetched: true,
                error: action.payload
            }
        default:
            return state;
    }
}