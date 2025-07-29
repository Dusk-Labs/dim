import React, { useCallback, useEffect } from "react";

import TimesCircleIcon from "../../assets/Icons/TimesCircle";
import UserIcon from "../../assets/Icons/User";
import KeyIcon from "../../assets/Icons/Key";

interface Props {
  name: string;
  icon: string;
  data: [string, React.Dispatch<React.SetStateAction<string>>];
  error: [string, React.Dispatch<React.SetStateAction<string>>];
  type?: string;
  placeholder?: string;
  autocomplete?: string;
  maxLength?: string;
}

function Field({
  name,
  icon,
  data,
  error,
  type = "text",
  placeholder = "",
  autocomplete = "off",
  maxLength,
}: Props) {
  const [value, setValue] = data;
  const [err, setErr] = error;

  useEffect(() => setErr(""), [setErr, value]);

  const handleOnChange = useCallback(
    (e) => {
      const newValue = e.target.value;

      if (type === "number") {
        setValue(parseInt(newValue, 10).toString());
        return;
      }

      setValue(newValue);
    },
    [setValue, type]
  );

  return (
    <label>
      <div className="name">
        {icon === "user" && <UserIcon />}
        {icon === "key" && <KeyIcon />}
        <p>{name}</p>
        {err && (
          <div className="horizontal-err">
            <TimesCircleIcon />
            <p>{err}</p>
          </div>
        )}
      </div>
      <input
        maxLength={parseInt(maxLength ?? "524288", 10)}
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
