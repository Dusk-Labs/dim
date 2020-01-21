import {
    AUTH_LOGIN_START,
    AUTH_LOGIN_OK,
    AUTH_LOGIN_ERR,
    AUTH_UPDATE_TOKEN,
    AUTH_LOGOUT,
    AUTH_REGISTER_ERR,
    AUTH_REGISTER_OK,
    AUTH_REGISTER_START,
    AUTH_CHECK_ADMIN_OK,
} from "../actions/types.js";

const initialState = {
    token: null,
    logging_in: false,
    logged_in: false,
    error: null,
    admin_exists: false,
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
        case AUTH_UPDATE_TOKEN:
            return {
                token: action.payload,
                logging_in: false,
                logged_in: true,
                error: null,
            }
        case AUTH_LOGOUT:
            return initialState;
        case AUTH_REGISTER_OK:
            return initialState;
        case AUTH_REGISTER_START:
            return {
                ...initialState,
                logging_in: true,
            };
        case AUTH_REGISTER_ERR:
            return {
                ...state,
                logging_in: false,
                logged_in: false,
                error: action.payload,
            };
        case AUTH_CHECK_ADMIN_OK:
            return {
                ...state,
                admin_exists: action.payload.exists,
            };
        default:
            return state;
    }
}
