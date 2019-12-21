import {
    AUTH_LOGIN_START,
    AUTH_LOGIN_OK,
    AUTH_LOGIN_ERR
} from "../actions/types.js";

const initialState = {
    token: null,
    logging_in: false,
    logged_in: false,
    error: null,
};

export default function(state = initialState, action) {
    switch(action.type) {
        case AUTH_LOGIN_START:
            return {
                token: null,
                logging_in: true,
                logged_in: false,
                error: null,
            };
        case AUTH_LOGIN_ERR:
            return {
                ...state,
                logging_in: false,
                logged_in: false,
                error: action.payload,
            };
        case AUTH_LOGIN_OK:
            return {
                ...state,
                token: action.payload.token,
                logged_in: true,
                logging_in: false,
            };
        default:
            return state;
    }
}
