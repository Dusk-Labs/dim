import {
    FETCH_BANNERS_START,
    FETCH_BANNERS_OK,
    FETCH_BANNERS_ERR
} from "../actions/types.js";

const initialState = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

export default function bannerReducer(state = initialState, action) {
    switch(action.type) {
        case FETCH_BANNERS_START:
            return {
                items: [],
                fetching: true,
                fetched: false,
                error: null
            }
        case FETCH_BANNERS_OK:
            return {
                ...state,
                fetching: false,
                fetched: true,
                items: action.payload
            }
        case FETCH_BANNERS_ERR:
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