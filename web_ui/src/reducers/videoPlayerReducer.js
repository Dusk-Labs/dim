import {
    TRANSCODE_START,
    TRANSCODE_OK,
    TRANSCODE_ERR,
    FETCH_FILE_START,
    FETCH_FILE_OK,
    FETCH_FILE_ERR
} from "../actions/types.js";

const start_transcode = {
    uuid: "",
    fetching: false,
    fetched: false,
    error: null
};

const fetch_file = {
    info: "",
    fetching: false,
    fetched: false,
    error: null
};


const initialState = {
    start_transcode,
    fetch_file
};

export default function(state = initialState, action) {
    switch(action.type) {
        case TRANSCODE_START:
            return {
                ...state,
                start_transcode: {
                    uuid: "",
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case TRANSCODE_OK:
            return {
                ...state,
                start_transcode: {
                    uuid: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case TRANSCODE_ERR:
            return {
                ...state,
                start_transcode: {
                    uuid: "",
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        case FETCH_FILE_START:
            return {
                ...state,
                fetch_file: {
                    info: "",
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case FETCH_FILE_OK:
            return {
                ...state,
                fetch_file: {
                    info: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case FETCH_FILE_ERR:
            return {
                ...state,
                fetch_file: {
                    info: "",
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        default:
            return state;
    }
}