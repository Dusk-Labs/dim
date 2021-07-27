import React from "react";
import { useCallback } from "react";
import { useDispatch } from "react-redux";
import { notificationsRemove } from "../actions/notifications";

import "./Toast.scss";

function Toast(props) {
  const dispatch = useDispatch();

  const dismiss = useCallback(() => {
    dispatch(notificationsRemove(props.id));
  }, [dispatch, props.id]);

  return (
    <div className="toast">
      {props.children}
      <button onClick={dismiss}>Dismiss</button>
    </div>
  );
}

export default Toast;
