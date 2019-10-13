import {
    FETCH_HOSTS_START,
    FETCH_HOSTS_OK,
    FETCH_HOSTS_ERR
} from "./types.js";

export const fetchHosts = () => async (dispatch) => {
    dispatch({ type: FETCH_HOSTS_START });

    try {
        // ! FOR WHEN API READY
        // const res = await fetch("");

        // if (res.status !== 200) {
        //     return dispatch({
        //         type: FETCH_HOSTS_ERR,
        //         payload: res.statusText
        //     });
        // }

        // const hosts = await res.json();
        // !

        const hosts = [
            { name: "Desktop", id: "1"},
            { name: "Laptop", id: "2"},
            { name: "Phone", id: "3"}
        ];

        dispatch({
            type: FETCH_HOSTS_OK,
            payload: hosts
        });
    } catch(err) {
        dispatch({
            type: FETCH_HOSTS_ERR,
            payload: err
        });
    }
};
