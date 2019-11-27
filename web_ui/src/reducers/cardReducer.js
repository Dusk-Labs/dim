import {
    FETCH_CARDS_START,
    FETCH_CARDS_OK,
    FETCH_CARDS_ERR
 } from "../actions/types.js";

const initialState = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_CARDS_START:
            return {
                items: [],
                fetching: true,
                fetched: false,
                error: null
            }
        case FETCH_CARDS_OK:
            return {
                items: action.payload,
                fetching: false,
                fetched: true,
                error: null
            }
        case FETCH_CARDS_ERR:
            return {
                items: [],
                fetching: false,
                fetched: true,
                error: action.payload
            }
        default:
            return state;
    }
}