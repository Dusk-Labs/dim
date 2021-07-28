import { useCallback } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch } from "react-redux";

import InputConfirmationBox from "../../Modals/InputConfirmationBox.jsx";
import { delAccount, logout } from "../../actions/auth.js";
import { useState } from "react";

function DelAccountBtn() {
  const [pass, setPass] = useState("");
  const [passErr, setPassErr] = useState("");

  const dispatch = useDispatch();
  const history = useHistory();

  // TODO: prevent logout if deleting account fails.
  const confirmDel = useCallback(async () => {
    if (pass.length === 0) {
      setPassErr("Enter your password to continue");
      return false;
    }

    await dispatch(delAccount(pass));
    await dispatch(logout());

    history.push("/login");
  }, [dispatch, history, pass]);

  return (
    <InputConfirmationBox
      title="Confirm action"
      cancelText="Nevermind"
      confirmText="Delete my account"
      action={confirmDel}
      msg="You are about to delete your account, are you sure you want to continue?"
      data={pass}
      setData={setPass}
      err={passErr}
      setErr={setPassErr}
      label="Password"
      type="password"
      icon="key"
    >
      <button className="critical">
        <p className="logout">Delete account</p>
      </button>
    </InputConfirmationBox>
  );
}

export default DelAccountBtn;
