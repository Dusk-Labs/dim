import { useCallback, useEffect } from "react";

import TimesCircleIcon from "../../assets/Icons/TimesCircle";
import UserIcon from "../../assets/Icons/User";
import KeyIcon from "../../assets/Icons/Key";

function Field(
  { name, icon, data, error, type = "text", placeholder = "", autocomplete = "off", maxLength }
) {
  const [value, setValue] = data;
  const [err, setErr] = error;

  useEffect(() => setErr(""), [setErr, value]);

  const handleOnChange = useCallback((e) => {
    const newValue = e.target.value;

    if (type === "number") {
      setValue(parseInt(newValue));
      return;
    }

    setValue(newValue);
  }, [setValue, type]);

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
        maxLength={maxLength || "524288"}
        placeholder={placeholder}
        onChange={handleOnChange}
        value={value}
        spellCheck="false"
        autoComplete={autocomplete}
        autoCorrect="off"
        autoCapitalize="none"
        type={type === "number" ? "text" : type}
      />
    </label>
  );
}

export default Field;
