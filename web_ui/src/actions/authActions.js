import {
    AUTH_OK,
    AUTH_ERR,
    AUTH_LOGOUT,
    START_AUTH
} from "./types";

export const authenticate = (username, password) => async (dispatch) => {
    dispatch({ type: START_AUTH });

    const config = {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "username": username,
            "password": password,
        })
    };

    try {
        const res = await fetch(`//${window.host}:8000/api/v1/auth/login`, config);

        if (res.status !== 200) {
            return dispatch({
                type: AUTH_ERR,
                payload: res.statusText,
            });
        }

        const payload = await res.json();

        dispatch({
            type: AUTH_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: AUTH_ERR,
            payload: err
        });
    }
};

export const logout = () => async (dispatch) => {
    dispatch({
        type: AUTH_LOGOUT
    });
}
