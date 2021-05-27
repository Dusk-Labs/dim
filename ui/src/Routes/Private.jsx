import { useEffect } from "react";
import { Route, useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import { updateAuthToken } from "../actions/auth.js";

function PrivateRoute(props) {
  const dispatch = useDispatch();
  const auth = useSelector(store => store.auth);

  const history = useHistory();
  const tokenInCookie = document.cookie.split("=")[1];

  const { logged_in, error } = auth.login;
  const { token } = auth;

  useEffect(() => {
    if (tokenInCookie && !token) {
      dispatch(updateAuthToken(tokenInCookie));
      return;
    }

    if (logged_in && token && !error && !tokenInCookie) {

      const dateExpires = new Date();

      dateExpires.setTime(dateExpires.getTime() + 604800000);

      document.cookie = (
        `token=${token};expires=${dateExpires.toGMTString()};`
      );
    }

    if (!token && !tokenInCookie) {
      history.push("/login");
    }
  }, [error, history, logged_in, token, tokenInCookie, dispatch]);

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

  const { exact, path, render, children } = props;

  return (token && tokenInCookie) && (
    <Route exact={exact} path={path} render={render} children={children}/>
  );
}

export default PrivateRoute;
