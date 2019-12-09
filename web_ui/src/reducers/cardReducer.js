import {
    FETCH_CARDS_START,
    FETCH_CARDS_OK,
    FETCH_CARDS_ERR,
    FETCH_CARD_START,
    FETCH_CARD_OK,
    FETCH_CARD_ERR
} from "../actions/types.js";

const fetch_cards = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const fetch_card = {
    info: {},
    fetching: false,
    fetched: false,
    error: null
};


const initialState = {
    fetch_cards,
    fetch_card
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_CARDS_START:
            return {
                ...state,
                fetch_cards: {
                    items: [],
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_CARDS_OK:
            return {
                ...state,
                fetch_cards: {
                    items: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_CARDS_ERR:
            return {
                ...state,
                fetch_cards: {
                    items: [],
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_CARD_START:
            return {
                ...state,
                fetch_card: {
                    info: {},
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_CARD_OK:
            return {
                ...state,
                fetch_card: {
                    info: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_CARD_ERR:
            return {
                ...state,
                fetch_card: {
                    info: {},
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        default:
            return state;
    }
}