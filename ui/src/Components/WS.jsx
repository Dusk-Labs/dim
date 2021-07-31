import React, { useEffect, useState } from "react";
import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";
import { notificationsAdd } from "../actions/notifications";

import { wsConnect, wsShowReconnect } from "../actions/ws";
import DimLogo from "../assets/DimLogo";
import Bar from "../Components/Load/Bar";

import "./WS.scss";

// initialize websocket connection for entire app
function WS(props) {
  const dispatch = useDispatch();

  const ws = useSelector(state => state.ws);
  const auth = useSelector(state => state.auth);

  const [tryingAgainIn, setTryingAgainIn] = useState(5);
  const [silentConnect, setSilentConnect] = useState(false);
  const [intervalID, setIntervalID] = useState();
  const [msg, setMsg] = useState("Connection failed");
  const [tries, setTries] = useState(0);

  const retry = useCallback(() => {
    dispatch(wsConnect());
    setMsg("Connection failed");
    setTries(count => count + 1);
    setTryingAgainIn(5);
    clearInterval(intervalID);
    setIntervalID();
  }, [dispatch, intervalID]);

  const handleClose = useCallback((e) => {
    if (e.wasClean) return;

    dispatch(notificationsAdd({
      msg: "Connection to server lost, some actions might not work."
    }));

    dispatch(wsShowReconnect());

    setTries(0);
    setSilentConnect(true);
    dispatch(wsConnect());
  }, [dispatch]);

  const handleOpen = useCallback((e) => {
    if (!silentConnect) return;
    setSilentConnect(false);
  }, [silentConnect]);

  useEffect(() => {
    if (ws.error && !intervalID) {
      const id = setInterval(() => {
        setTryingAgainIn(state => state - 1);
      }, 1000);

      setIntervalID(id);
    }

    return () => {
      if (!intervalID) return;
      clearInterval(intervalID);
    };
  }, [intervalID, ws.error]);

  useEffect(() => {
    console.log(tryingAgainIn);

    if (tryingAgainIn <= 0) {
      retry();
    }
  }, [intervalID, retry, tryingAgainIn]);

  useEffect(() => {
    dispatch(wsConnect());
  }, [dispatch]);

  useEffect(() => {
    if (!ws.conn) return;

    ws.conn.addEventListener("open", handleOpen);
    ws.conn.addEventListener("close", handleClose);

    return () => {
      ws.conn.removeEventListener("open", handleOpen);
      ws.conn.removeEventListener("close", handleClose);
    };
  }, [handleClose, handleOpen, ws.conn]);

  useEffect(() => {
    if (!auth.token || !ws.conn) return;

    const payload = {
      "type": "authenticate",
      "token": auth.token
    };

    ws.conn.send(JSON.stringify(payload));
  }, [auth.token, ws.conn]);

  if (!silentConnect && (ws.connecting || ws.error)) {
    return (
      <div className="appLoad showAfter100ms">
        <DimLogo load/>
        {ws.error && (
          <div className="error">
            <h2>{msg}</h2>
            {tries > 0 && <p>Seems like maybe the server is offline</p>}
            <button onClick={retry}>Try reconnect ({tryingAgainIn})</button>
          </div>
        )}
        {!ws.error && (
          <Bar/>
        )}
      </div>
    );
  }

  if ((ws.connected && !ws.error) || silentConnect) {
    return props.children;
  }

  return (
    <div className="appLoad showAfter100ms">
      <DimLogo load/>
      <Bar/>
    </div>
  );
}

export default WS;
