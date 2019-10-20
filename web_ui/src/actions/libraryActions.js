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
    DEL_LIBRARY_ERR
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

export const newLibrary = (data) => async (dispatch) => {
    dispatch({ type: NEW_LIBRARY_START });

    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    };

    try {
        const res = await fetch(`http://86.21.150.167:8000/api/v1/library`, options);

        if (res.status !== 201) {
            return dispatch({
                type: NEW_LIBRARY_ERR,
                payload: res.statusText
            });
        }

        dispatch({ type: NEW_LIBRARY_OK });
    } catch(err) {
        dispatch({
            type: NEW_LIBRARY_ERR,
            payload: err
        });
    }
};

export const delLibrary = (id) => async (dispatch) => {
    dispatch({ type: DEL_LIBRARY_START });

    const options = {
        method: "DELETE"
    };

    try {
        const res = await fetch(`http://86.21.150.167:8000/api/v1/library/${id}`, options);

        if (res.status !== 204) {
            return dispatch({
                type: DEL_LIBRARY_ERR,
                payload: res.statusText
            });
        }

        dispatch({ type: DEL_LIBRARY_OK });
    } catch(err) {
        dispatch({
            type: DEL_LIBRARY_ERR,
            payload: err
        });
    }
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