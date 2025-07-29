import { useCallback } from "react";

import "./Button.scss";

type ButtonProps = {
  disabled?: boolean;
  onClick?: (...args: any) => any;
  type?: string;
  children: React.ReactNode;
};

function Button(props: ButtonProps) {
  const handleClick = useCallback(() => {
    if (props.disabled) return;

    if (props.onClick) {
      props.onClick();
    }
  }, [props]);

  return (
    <button
      className={`btn ${props.type || "primary"} ${
        props.disabled ? "disabled" : ""
      }`}
      onClick={handleClick}
    >
      {props.children}
    </button>
  );
}

export default Button;
