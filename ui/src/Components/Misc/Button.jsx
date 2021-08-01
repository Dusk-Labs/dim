import { useCallback } from "react";

import "./Button.scss";

function Button(props) {
  const handleClick = useCallback(() => {
    if (props.disabled) return;

    if (props.onClick) {
      props.onClick();
    }
  }, [props]);

  return (
    <button
      title={`${props.disabled ? "This feature is currently disabled." : ""}`}
      className={`btn ${props.type || "primary"} ${props.disabled ? "disabled" : ""}`}
      onClick={handleClick}
    >
      {props.children}
    </button>
  );
}

export default Button;
