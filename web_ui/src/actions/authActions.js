import {
    AUTH_LOGIN_START,
    AUTH_LOGIN_ERR,
    AUTH_LOGIN_OK,
    AUTH_UPDATE_TOKEN,
    AUTH_LOGOUT,
    AUTH_REGISTER_START,
    AUTH_REGISTER_ERR,
    AUTH_REGISTER_OK,
    AUTH_CHECK_ADMIN_ERR,
    AUTH_CHECK_ADMIN_OK,
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

export const logout = () => async (dispatch) => {
    // Remove all cookies
    // FIXME: Is this the proper place to place shit like this?
    document.cookie = "token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";

    dispatch({
        type: AUTH_LOGOUT
    });
};

export const updateAuthToken = (token) => (dispatch) => {
    dispatch({
        type: AUTH_UPDATE_TOKEN,
        payload: token
    });
};

export const register = (username, password, invite) => async (dispatch) => {
    dispatch({ type: AUTH_REGISTER_START });

    const config = {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "username": username,
            "password": password,
            "invite_token": invite
        })
    };

    try {
        const res = await fetch(`//${window.host}:8000/api/v1/auth/register`, config);
        const payload = await res.json();


        if (res.status !== 200) {
            return dispatch({
                type: AUTH_REGISTER_ERR,
                payload: res.statusText,
            });
        } else if (!!payload.error) {
            return dispatch({
                type: AUTH_REGISTER_ERR,
                payload: payload.error
            });
        }

        dispatch({
            type: AUTH_REGISTER_OK,
        });
    } catch(err) {
        dispatch({
            type: AUTH_REGISTER_ERR,
            payload: err,
        });
    }
};

export const checkAdminExists = () => async (dispatch) => {
    try {
        const res = await fetch(`//${window.host}:8000/api/v1/auth/admin_exists`);
        if (res.status !== 200)
            return dispatch({
                type: AUTH_CHECK_ADMIN_ERR,
                payload: res.statusText
            })

        const payload = await res.json();

        dispatch({
            type: AUTH_CHECK_ADMIN_OK,
            payload: payload,
        });
    } catch {
    }
}
