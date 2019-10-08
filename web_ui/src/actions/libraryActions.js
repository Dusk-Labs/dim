import {
    FETCH_LIBRARIES_START,
    FETCH_LIBRARIES_OK,
    FETCH_LIBRARIES_ERR,
    FETCH_LIBRARY_INFO,
    FETCH_LIBRARY_MEDIA,
    NEW_LIBRARY,
    DEL_LIBRARY
} from "./types.js";

export const fetchLibraries = () => async (dispatch) => {
    dispatch({ type: FETCH_LIBRARIES_START });

    try {
        const res = await fetch("http://86.21.150.167:8000/api/v1/library");

        if (res.status !== 200) {
            return dispatch({
                type: FETCH_LIBRARIES_ERR,
                payload: res.statusText
            });
        }

        const libs = await res.json();

        dispatch({
            type: FETCH_LIBRARIES_OK,
            payload: libs
        });
    } catch(err) {
        dispatch({
            type: FETCH_LIBRARIES_ERR,
            payload: err
        });
    }
};

export const newLibrary = () => async (dispatch) => {
    dispatch({
        type: NEW_LIBRARY
    });
};

export const fetchLibraryInfo = () => async (dispatch) => {
    dispatch({
        type: FETCH_LIBRARY_INFO
    });
};

export const fetchLibraryMedia = () => async (dispatch) => {
    dispatch({
        type: FETCH_LIBRARY_MEDIA
    });
};

export const delLibrary = () => async (dispatch) => {
    dispatch({
        type: DEL_LIBRARY
    });
};