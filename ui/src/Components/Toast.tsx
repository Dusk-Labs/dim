import { useCallback } from "react";

import { useAppDispatch } from "../hooks/store";
import { removeNotification } from "../slices/notifications";

import "./Toast.scss";

type ToastProps = {
  id: number;
  children: React.ReactNode;
};

function Toast(props: ToastProps) {
  const dispatch = useAppDispatch();

  const dismiss = useCallback(() => {
    dispatch(removeNotification(props.id));
  }, [dispatch, props.id]);

  return (
    <div className="toast">
      {props.children}
      <button onClick={dismiss}>Dismiss</button>
    </div>
  );
}

export default Toast;
