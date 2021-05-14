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

  const [msg, setMsg] = useState("Connection Failed");
  const [tries, setTries] = useState(0);

  const retry = useCallback(() => {
    dispatch(wsConnect());
    setMsg("Connection Failed");
    setTries(count => count + 1);
  }, [dispatch]);

  const handleClose = useCallback((e) => {
    if (e.wasClean) return;
    setMsg("Connection Lost");
    setTries(0);
    dispatch(wsConnect());
  }, [dispatch]);

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
              <button onClick={retry}>Try reconnect</button>
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
