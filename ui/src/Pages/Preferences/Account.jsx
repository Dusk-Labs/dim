import { useCallback, useEffect, useState } from "react";
import Field from "../Auth/Field";

import "./Account.scss";

function MyAccount() {
  const [oldPass, setOldPass] = useState("");
  const [oldPassErr, setOldPassErr] = useState("");
  const [newPass, setNewPass] = useState("");
  const [newPassErr, setNewPassErr] = useState("");

  const [valid, setValid] = useState(false);

  const changePass = useCallback(() => {
    if (!valid) return;
  }, [valid]);

  useEffect(() => {
    setValid(oldPass.length > 4 && newPass.length > 4);
  }, [newPass.length, oldPass.length]);

  return (
    <div className="preferencesAccount">
      <section>
        <h2>Manage password</h2>
        <div className="fields">
          <Field
            name="Current password"
            icon="key"
            data={[oldPass, setOldPass]}
            error={[oldPassErr, setOldPassErr]}
            type="password"
          />
          <Field
            name="New password"
            icon="key"
            data={[newPass, setNewPass]}
            error={[newPassErr, setNewPassErr]}
            type="password"
          />
        </div>
        <button className={`${!valid && "disabled"}`} onClick={changePass}>
          Change password
        </button>
      </section>
      <section>
        <h2>Manage account</h2>
        <p className="desc">Your actual media on the system does not get deleted.</p>
        <div className="options">
          <button className="critical">Delete account</button>
        </div>
      </section>
    </div>
  );
}

export default MyAccount;
