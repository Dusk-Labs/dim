import { useCallback } from "react";

type ButtonProps = {
  onClick: () => void;
  children: any;
  className: string | null;
};

// this component allows you to blur a button and make it unfocusable, this is done by creating a component and calling it on the files that are needed

export function UnfocusableButton(props: ButtonProps) {
  const { onClick, children, className } = props;

  const callback = useCallback(
    (e) => {
      onClick();
      e.target.blur();
    },
    [onClick]
  );

  return (
    <button className={`unfocusableButton ${className}`} onClick={callback}>
      {children}
    </button>
  );
}
