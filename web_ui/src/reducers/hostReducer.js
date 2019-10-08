import {
    FETCH_HOSTS_START,
    FETCH_HOSTS_OK,
    FETCH_HOSTS_ERR
 } from "../actions/types.js";

const initialState = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_HOSTS_START:
            return {
                ...state,
                fetching: true
            }
        case FETCH_HOSTS_OK:
            return {
                ...state,
                fetching: false,
                fetched: true,
                items: action.payload
            }
        case FETCH_HOSTS_ERR:
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