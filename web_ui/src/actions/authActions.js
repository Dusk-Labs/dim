import {
    AUTH_LOGIN_START,
    AUTH_LOGIN_ERR,
    AUTH_LOGIN_OK
} from "./types";

export const authenticate = (username, password) => async (dispatch) => {
    dispatch({ type: AUTH_LOGIN_START });

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
                type: AUTH_LOGIN_ERR,
                payload: res.statusText,
            });
        }

        const payload = await res.json();

        dispatch({
            type: AUTH_LOGIN_OK,
            payload
        });
    } catch(err) {
        dispatch({
            type: AUTH_LOGIN_ERR,
            payload: err
        });
    }
};

export const logout = () => async (dispatch) => {};
