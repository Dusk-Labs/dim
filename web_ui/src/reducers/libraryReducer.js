import {
    FETCH_LIBRARIES_START,
    FETCH_LIBRARIES_OK,
    FETCH_LIBRARIES_ERR,
    FETCH_LIBRARY_INFO,
    FETCH_LIBRARY_MEDIA,
    NEW_LIBRARY,
    DEL_LIBRARY
} from "../actions/types.js";

const initialState = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_LIBRARIES_START:
            return {
                ...state,
                fetching: true
            }
        case FETCH_LIBRARIES_OK:
            return {
                ...state,
                fetching: false,
                fetched: true,
                items: action.payload
            }
        case FETCH_LIBRARIES_ERR:
            return {
                ...state,
                fetching: false,
                fetched: true,
                error: action.payload
            }
        case FETCH_LIBRARY_INFO:
            return state;
        case FETCH_LIBRARY_MEDIA:
            return state;
        case NEW_LIBRARY:
            return state;
        case DEL_LIBRARY:
            return state;
        default:
            return state;
    }
}