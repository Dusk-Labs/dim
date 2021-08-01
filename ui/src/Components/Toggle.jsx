import { useCallback, useEffect, useState } from "react";
import "./Toggle.scss";

function Toggle(props) {
  const [active, setActive] = useState(false);

  useEffect(() => {
    if (props.state === undefined) return;
    setActive(props.state);
  }, [props.state]);

  const toggle = useCallback(() => {
    if (props.disabled) return;

    if (props.onToggle) {
      props.onToggle(!active);
    }

    setActive(state => !state);
  }, [active, props]);

  return (
    <div className={`toggleContainer disabled-${props.disabled}`}>
      <p>{props.name}</p>
      {props.desc && <p className="desc">{props.desc}</p>}
      <div
        onClick={toggle}
        className={`toggle active-${active}`}
      >
        <div className="ball"/>
      </div>
    </div>
  );
}

export default Toggle;
