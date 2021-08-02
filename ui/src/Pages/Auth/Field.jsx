import { useEffect } from "react";

import TimesCircleIcon from "../../assets/Icons/TimesCircle";
import UserIcon from "../../assets/Icons/User";
import KeyIcon from "../../assets/Icons/Key";

function Field(
  { name, icon, data, error, type = "text", placeholder = "", autocomplete = "off" }
) {
  const [value, setValue] = data;
  const [err, setErr] = error;

  useEffect(() => setErr(""), [setErr, value]);

  return (
    <label>
      <div className="name">
        {icon === "user" && <UserIcon/>}
        {icon === "key" && <KeyIcon/>}
        <p>{name}</p>
        {err && (
          <div className="horizontal-err">
            <TimesCircleIcon/>
            <p>{err}</p>
          </div>
        )}
      </div>
      <input
        placeholder={placeholder}
        onChange={e => setValue(e.target.value)}
        value={value}
        spellCheck="false"
        autoComplete={autocomplete}
        autoCorrect="off"
        autoCapitalize="none"
        type={type}
      />
    </label>
  );
}

export default Field;
