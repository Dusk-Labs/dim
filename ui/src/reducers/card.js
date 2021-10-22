import {
  FETCH_CARDS_START,
  FETCH_CARDS_OK,
  FETCH_CARDS_ERR
} from "../actions/types";

const cards = {
  items: [],
  fetching: false,
  fetched: false,
  error: null
};

const initialState = {
  cards
};

export default function cardReducer(state = initialState, action) {
  switch(action.type) {
    case FETCH_CARDS_START:
      return {
        ...state,
        cards: {
          ...cards,
          items: [],
          fetching: true,
          fetched: false,
          error: null
        }
      };
    case FETCH_CARDS_OK:
      return {
        ...state,
        cards: {
          ...cards,
          items: action.payload,
          fetching: false,
          fetched: true,
          error: null
        }
      };
    case FETCH_CARDS_ERR:
      return {
        ...state,
        cards: {
          ...cards,
          items: [],
          fetching: false,
          fetched: true,
          error: action.payload
        }
      };
    default:
      return state;
  }
}
