import { useCallback, useEffect, useState } from "react";
import { useDispatch } from "react-redux";

import { changePassword } from "../../../actions/auth";
import Button from "../../../Components/Misc/Button";
import Field from "../../Auth/Field";

function ManagePassword() {
  const dispatch = useDispatch();

  const [oldPass, setOldPass] = useState("");
  const [oldPassErr, setOldPassErr] = useState("");
  const [newPass, setNewPass] = useState("");
  const [newPassErr, setNewPassErr] = useState("");

  const [valid, setValid] = useState(false);

  const changePass = useCallback(async () => {
    if (!valid) return;

    if (oldPass === newPass) {
      setNewPassErr("Your new password is the same as your current password.");
      return;
    }

    await dispatch(changePassword(oldPass, newPass));

    setOldPass("");
    setNewPass("");
  }, [dispatch, newPass, oldPass, valid]);

  const cancelChangePass = useCallback(async () => {
    setOldPass("");
    setNewPass("");
  }, []);

  useEffect(() => {
    setValid(oldPass.length > 4 && newPass.length > 4);
  }, [newPass.length, oldPass.length]);

  return (
    <section>
      <h2>Manage password</h2>
      <div className="fields">
        <Field
          name="Current password"
          icon="key"
          data={[oldPass, setOldPass]}
          error={[oldPassErr, setOldPassErr]}
          type="password"
          autocomplete="current-password"
        />
        <Field
          name="New password"
          icon="key"
          data={[newPass, setNewPass]}
          error={[newPassErr, setNewPassErr]}
          type="password"
          autocomplete="new-password"
        />
      </div>
      {valid && (
        <div className="options">
          <Button disabled={!valid} onClick={changePass}>
            Change password
          </Button>
          <Button type="secondary" onClick={cancelChangePass}>
            Cancel
          </Button>
        </div>
      )}
    </section>
  );
}

export default ManagePassword;
