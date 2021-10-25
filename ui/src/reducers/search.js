import {
  QUICK_SEARCH_START,
  QUICK_SEARCH_OK,
  QUICK_SEARCH_ERR,
  SEARCH_START,
  SEARCH_OK,
  SEARCH_ERR
} from "../actions/types";

const search = {
  items: [],
  fetching: false,
  fetched: false,
  error: null
};

const quick_search = {
  items: [],
  fetching: false,
  fetched: false,
  error: null
};

const initialState = {
  search,
  quick_search
};

export default function searchReducer(state = initialState, action) {
  switch(action.type) {
    case SEARCH_START:
      return {
        ...state,
        search: {
          items: [],
          fetching: true,
          fetched: false,
          error: null
        }
      };
    case SEARCH_OK:
      return {
        ...state,
        search: {
          ...search,
          items: {"RESULTS": action.payload},
          fetching: false,
          fetched: true
        }
      };
    case SEARCH_ERR:
      return {
        ...state,
        search: {
          ...search,
          fetching: false,
          fetched: true,
          error: action.payload
        }
      };
    case QUICK_SEARCH_START:
      return {
        ...state,
        quick_search: {
          items: [],
          fetching: true,
          fetched: false,
          error: null
        }
      };
    case QUICK_SEARCH_OK:
      return {
        ...state,
        quick_search: {
          ...quick_search,
          items: action.payload,
          fetching: false,
          fetched: true
        }
      };
    case QUICK_SEARCH_ERR:
      return {
        ...state,
        quick_search: {
          ...quick_search,
          fetching: false,
          fetched: true,
          error: action.payload
        }
      };
    default:
      return state;
  }
}
