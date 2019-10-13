import {
    FETCH_USER_START,
    FETCH_USER_OK,
    FETCH_USER_ERR
} from "../actions/types.js";

const initialState = {
    info: {},
    fetching: false,
    fetched: false,
    error: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_USER_START:
            return {
                ...state,
                fetching: true
            }
        case FETCH_USER_OK:
            return {
                ...state,
                fetching: false,
                fetched: true,
                info: action.payload
            }
        case FETCH_USER_ERR:
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