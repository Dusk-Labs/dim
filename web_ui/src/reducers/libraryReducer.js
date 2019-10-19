import {
    FETCH_LIBRARIES_START,
    FETCH_LIBRARIES_OK,
    FETCH_LIBRARIES_ERR,
    FETCH_LIBRARY_INFO,
    FETCH_LIBRARY_MEDIA,
    NEW_LIBRARY_START,
    NEW_LIBRARY_OK,
    NEW_LIBRARY_ERR,
    DEL_LIBRARY_START,
    DEL_LIBRARY_OK,
    DEL_LIBRARY_ERR,
} from "../actions/types.js";

const fetch_libraries = {
    items: [],
    fetching: false,
    fetched: false,
    error: null
};

const new_library = {
    creating: false,
    created: false,
    error: null
};

const del_library = {
    deleting: false,
    deleted: false,
    error: null
};

const initialState = {
    fetch_libraries,
    new_library,
    del_library
};

export default function(state = initialState, action) {
    switch(action.type) {
        case FETCH_LIBRARIES_START:
            return {
                ...state,
                fetch_libraries: {
                    ...fetch_libraries,
                    fetching: true
                }
            }
        case FETCH_LIBRARIES_OK:
            return {
                ...state,
                fetch_libraries: {
                    ...fetch_libraries,
                    fetching: false,
                    fetched: true,
                    items: action.payload
                }
            }
        case FETCH_LIBRARIES_ERR:
            return {
                ...state,
                fetch_libraries: {
                    ...fetch_libraries,
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_LIBRARY_INFO:
            return state;
        case FETCH_LIBRARY_MEDIA:
            return state;
        case NEW_LIBRARY_START:
            return {
                ...state,
                new_library: {
                    ...new_library,
                    creating: true,
                }
            }
        case NEW_LIBRARY_OK:
            return {
                ...state,
                new_library: {
                    ...new_library,
                    creating: false,
                    created: true
                }
            }
        case NEW_LIBRARY_ERR:
            return {
                ...state,
                new_library: {
                    creating: false,
                    created: false,
                    error: action.payload
                }
            }
        case DEL_LIBRARY_START:
            return {
                ...state,
                del_library: {
                    ...del_library,
                    deleting: true,
                }
            }
        case DEL_LIBRARY_OK:
            return {
                ...state,
                del_library: {
                    ...del_library,
                    deleting: false,
                    deleted: true
                }
            }
        case DEL_LIBRARY_ERR:
            return {
                ...state,
                del_library: {
                    deleting: false,
                    deleted: false,
                    error: action.payload
                }
            }
        default:
            return state;
    }
}