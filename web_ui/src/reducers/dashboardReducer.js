import {
    FETCH_DASHBOARD_START,
    FETCH_DASHBOARD_OK,
    FETCH_DASHBOARD_ERR
} from "../actions/types.js";

const initialState = {
    sections: {},
    fetching: false,
    fetched: false,
    error: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_DASHBOARD_START:
            return {
                ...state,
                fetching: true
            }
        case FETCH_DASHBOARD_OK:
            return {
                ...state,
                fetching: false,
                fetched: true,
                sections: action.payload
            }
        case FETCH_DASHBOARD_ERR:
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