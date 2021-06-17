import React, { useEffect, useState } from "react";
import { useCallback } from "react";
import { useDispatch, useSelector } from "react-redux";

import { wsConnect } from "../actions/ws";
import DimLogo from "../assets/DimLogo";
import Bar from "../Components/Load/Bar";

import "./WS.scss";

// initialize websocket connection for entire app
function WS(props) {
  const dispatch = useDispatch();
  const ws = useSelector(state => state.ws);

  const [tryingAgainIn, setTryingAgainIn] = useState(10);
  const [intervalID, setIntervalID] = useState();
  const [msg, setMsg] = useState("Connection failed");
  const [tries, setTries] = useState(0);

  const retry = useCallback(() => {
    dispatch(wsConnect());
    setMsg("Connection failed");
    setTries(count => count + 1);
  }, [dispatch]);

  const handleClose = useCallback((e) => {
    if (e.wasClean) return;

    setMsg("Connection lost");
    setTries(0);
    dispatch(wsConnect());
  }, [dispatch]);

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
      setTryingAgainIn(10);
      clearInterval(intervalID);
      setIntervalID();
    }
  }, [intervalID, retry, tryingAgainIn]);

  useEffect(() => {
    dispatch(wsConnect());
  }, [dispatch]);

  useEffect(() => {
    if (!ws.conn) return;

    ws.conn.addEventListener("close", handleClose);
    return () => ws.conn.removeEventListener("close", handleClose);
  }, [handleClose, ws.conn]);

  return (
    <>
      {(ws.connecting || ws.error) && (
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
      )}
      {(ws.connected && !ws.error) && (
        props.children
      )}
    </>
  );
}

export default WS;
