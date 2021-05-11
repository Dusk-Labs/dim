import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";

import { wsConnect } from "../actions/ws";
import DimLogo from "../assets/DimLogo";
import Bar from "../Components/Load/Bar";

import "./WS.scss";

// initialize websocket connection for entire app
function WS(props) {
  const dispatch = useDispatch();
  const ws = useSelector(state => state.ws);

  useEffect(() => {
    dispatch(wsConnect());
  }, [dispatch]);

  return (
    <>
      {(ws.connecting || ws.error) && (
        <div className="appLoad showAfter100ms">
          <DimLogo load/>
          {ws.error && (
            <p>Connection Failed</p>
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
