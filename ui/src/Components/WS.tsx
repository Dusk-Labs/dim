import { createContext, useEffect, useState, useCallback } from "react";

import { addNotification } from "../slices/notifications";
import { useAppDispatch, useAppSelector } from "../hooks/store";
import { wsConnect, wsShowReconnect } from "../actions/ws";
import DimLogo from "../assets/DimLogo";
import Bar from "../Components/Load/Bar";

import "./WS.scss";

export const WebSocketContext = createContext<WebSocket | null>(null);

// initialize websocket connection for entire app
function WS(props: React.PropsWithChildren<{}>) {
  const dispatch = useAppDispatch();

  const ws = useAppSelector((state) => state.ws);
  const auth = useAppSelector((state) => state.auth);

  const [conn, setConn] = useState<WebSocket | null>(null);
  const [tryingAgainIn, setTryingAgainIn] = useState(5);
  const [silentConnect, setSilentConnect] = useState(false);
  const [intervalID, setIntervalID] = useState<number>();
  const [msg, setMsg] = useState("Connection failed");
  const [tries, setTries] = useState(0);

  const onNewSocket = (newSocket: WebSocket) => {
    setConn(newSocket);
  };

  const retry = useCallback(() => {
    dispatch(wsConnect(onNewSocket));
    setMsg("Connection failed");
    setTries((count) => count + 1);
    setTryingAgainIn(5);
    clearInterval(intervalID);
    setIntervalID(undefined);
  }, [dispatch, intervalID]);

  const handleClose = useCallback(
    (e) => {
      if (e.wasClean) return;

      dispatch(
        addNotification({
          msg: "Connection to server lost, some actions might not work.",
        })
      );

      dispatch(wsShowReconnect());

      setTries(0);
      setSilentConnect(true);
      dispatch(wsConnect(onNewSocket));
    },
    [dispatch]
  );

  const handleOpen = useCallback(() => {
    if (!silentConnect) return;
    setSilentConnect(false);
  }, [silentConnect]);

  useEffect(() => {
    if (ws.error && !intervalID) {
      const id = window.setInterval(() => {
        setTryingAgainIn((state) => state - 1);
      }, 1000);

      setIntervalID(id);
    }

    return () => {
      if (!intervalID) return;
      clearInterval(intervalID);
    };
  }, [intervalID, ws.error]);

  useEffect(() => {
    if (tryingAgainIn <= 0) {
      retry();
    }
  }, [intervalID, retry, tryingAgainIn]);

  useEffect(() => {
    dispatch(wsConnect(onNewSocket));
  }, [dispatch]);

  useEffect(() => {
    if (!conn) return;

    conn.addEventListener("open", handleOpen);
    conn.addEventListener("close", handleClose);

    return () => {
      conn.removeEventListener("open", handleOpen);
      conn.removeEventListener("close", handleClose);
    };
  }, [conn, handleClose, handleOpen]);

  useEffect(() => {
    if (!auth.token || !conn) return;

    const payload = {
      type: "authenticate",
      token: auth.token,
    };

    conn.send(JSON.stringify(payload));
  }, [auth.token, conn]);

  if (!silentConnect && (ws.connecting || ws.error)) {
    return (
      <div className="appLoad showAfter100ms">
        <DimLogo />
        {ws.error && (
          <div className="error">
            <h2>{msg}</h2>
            {tries > 0 && <p>Seems like maybe the server is offline</p>}
            <button onClick={retry}>Try reconnect ({tryingAgainIn})</button>
          </div>
        )}
        {!ws.error && (
          <>
            <h2>Connecting to server</h2>
            <Bar />
          </>
        )}
      </div>
    );
  }

  if ((ws.connected && !ws.error) || silentConnect) {
    return (
      <WebSocketContext.Provider value={conn}>
        {props.children}
      </WebSocketContext.Provider>
    );
  }

  return (
    <div className="appLoad showAfter100ms">
      <DimLogo />
      <Bar />
    </div>
  );
}

export default WS;
