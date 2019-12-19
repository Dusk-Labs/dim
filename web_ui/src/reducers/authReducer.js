import {
    AUTH_OK,
    AUTH_ERR,
    AUTH_LOGOUT,
    START_AUTH
} from "../actions/types.js";

const initialState = {
    logged_in: false,
    error: null,
    fetched: false,
    fetching: false,
    token: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case AUTH_OK:
            return {
                ...state,
                token: action.payload.token,
                fetched: true,
                fetching: false,
                logged_in: true
            };
        case AUTH_ERR:
            return {
                ...state,
                fetching: false,
                fetched: true,
                error: action.payload,
                logged_in: false,
            };
        case START_AUTH:
            return {
                ...state,
                fetching: true,
                fetched: false,
            };
        case AUTH_LOGOUT:
            return {
                logged_in: false,
                error: null,
                fetched: false,
                fetching: false,
                token: null
            };
        default:
            return state;
    }
}
