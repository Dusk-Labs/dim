import { useEffect } from "react";
import { Route, useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import { checkAdminExists, updateAuthToken } from "../actions/auth.js";
import { fetchUser } from "../actions/user.js";

function PrivateRoute(props) {
  const dispatch = useDispatch();

  const { auth, user } = useSelector((store) => ({
    auth: store.auth,
    user: store.user,
  }));

  const history = useHistory();

  const tokenInCookie = document?.cookie
    .split("; ")
    .find((cookie) => cookie.startsWith("token="))
    ?.split("=")[1];

  const { logged_in, error } = auth.login;
  const { token, admin_exists } = auth;

  useEffect(() => {
    if (tokenInCookie && !token) {
      dispatch(updateAuthToken(tokenInCookie));
      return;
    }

    // save token in cookie if not saved
    if (logged_in && token && !error && !tokenInCookie) {
      const dateExpires = new Date();

      // expire cookie token in 7 days
      dateExpires.setTime(dateExpires.getTime() + 604800000);

      document.cookie = `token=${token};expires=${dateExpires.toGMTString()};samesite=lax;`;
    }

    if (!token && !tokenInCookie) {
      switch (admin_exists) {
        case true:
          history.push("/login");
          return;
        case false:
          history.push("/register");
          return;
        default:
          return;
      }
    }
  }, [error, history, logged_in, token, tokenInCookie, admin_exists, dispatch]);

  // auto logout when logged out in another tab
  useEffect(() => {
    if (!("BroadcastChannel" in window)) return;

    const bc = new BroadcastChannel("dim");

    bc.onmessage = (e) => {
      if (document.hasFocus()) return;

      if (e.data === "logout") {
        /*
          cannot use history.push, throws an error when
          tab is not active and it tries to redirect.
        */
        window.location.replace("/login");
      }
    };

    return () => bc.close();
  }, []);

  /*
    scroll to top on route change
    (cannot enable smooth -> page doesn't go fully up)
  */
  useEffect(() => {
    window.scrollTo(0, 0);
  }, [history.location.pathname]);

  // clears any remaining video streams
  useEffect(() => {
    if (history.location.pathname.includes("/play/")) return;

    const GID = sessionStorage.getItem("GID");

    if (!GID) return;

    (async () => {
      await fetch(`/api/v1/stream/${GID}/state/kill`);
      sessionStorage.clear();
    })();
  }, [history.location.pathname]);

  useEffect(() => {
    dispatch(checkAdminExists());
    dispatch(fetchUser());
  }, [dispatch]);

  const { exact, path, render, children } = props;
  const userExists = user.fetched && !user.error;

  return (
    userExists &&
    token &&
    tokenInCookie && (
      <Route exact={exact} path={path} render={render} children={children} />
    )
  );
}

export default PrivateRoute;
