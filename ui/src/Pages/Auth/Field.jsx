import { useEffect } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

function Field(
  { name, icon, data, error, type = "text" }
) {
  const [ value, setValue ] = data;
  const [ err, setErr ] = error;

  useEffect(() => setErr(""), [setErr, value]);

  return (
    <label>
      <div className="name">
        <FontAwesomeIcon icon={icon}/>
        <p>{name}</p>
        {err && (
          <div className="horizontal-err">
            <FontAwesomeIcon icon="times-circle"/>
            <p>{err}</p>
          </div>
        )}
      </div>
      <input
        onChange={e => setValue(e.target.value)}
        value={value}
        spellCheck="false"
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="none"
        type={type}
      />
    </label>
  );
}

export default Field;
