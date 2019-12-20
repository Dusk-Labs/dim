import {
    TRANSCODE_START,
    TRANSCODE_OK,
    TRANSCODE_ERR,
    DEL_TRANSCODE_START,
    DEL_TRANSCODE_OK,
    DEL_TRANSCODE_ERR
} from "../actions/types.js";

const start_transcode = {
    uuid: "",
    fetching: false,
    fetched: false,
    error: null
};

const del_transcode = {
    data: {},
    fetching: false,
    fetched: false,
    error: null
};

const initialState = {
    start_transcode,
    del_transcode
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
                    uuid: action.payload.uuid,
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
        case DEL_TRANSCODE_START:
            return {
                ...state,
                del_transcode: {
                    data: {},
                    fetching: true,
                    fetched: false,
                    error: null
                }
            }
        case DEL_TRANSCODE_OK:
            return {
                ...state,
                del_transcode: {
                    data: action.payload,
                    fetching: false,
                    fetched: true,
                    error: null
                }
            }
        case DEL_TRANSCODE_ERR:
            return {
                ...state,
                del_transcode: {
                    data: {},
                    fetching: false,
                    fetched: true,
                    error: action.payload
                }
            }
        default:
            return state;
    }
}