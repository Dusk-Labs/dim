import {
    FETCH_USER_START,
    FETCH_USER_OK,
    FETCH_USER_ERR
} from "./types.js";

// ! REMOVE AFTER API IMPLEMENTED
import UserIcon from "../assets/profile_icon.jpg";
// !

export const fetchUser = () => async (dispatch) => {
    dispatch({ type: FETCH_USER_START });

    try {
        // ! FOR WHEN API READY
        // const res = await fetch("");

        // if (res.status !== 200) {
        //     return dispatch({
        //         type: FETCH_PROFILE_ERR,
        //         payload: res.statusText
        //     });
        // }

        // const profile = await res.json();
        // !

        const profile = {
            username: "Lana Rhoades",
            picture: UserIcon,
            spentWatching: 12
        };

        dispatch({
            type: FETCH_USER_OK,
            payload: profile
        });
    } catch {
        dispatch({
            type: FETCH_USER_ERR,
            payload: null
        });
    }
};
