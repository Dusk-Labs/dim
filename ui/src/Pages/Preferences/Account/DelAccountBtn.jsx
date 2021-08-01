import { useCallback, useEffect } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";

import InputConfirmationBox from "../../../Modals/InputConfirmationBox.jsx";
import { delAccount } from "../../../actions/auth.js";
import { useState } from "react";
import Button from "../../../Components/Misc/Button.jsx";

function DelAccountBtn() {
  const deleteAccount = useSelector(store => store.auth.delAccount);

  const [pass, setPass] = useState("");
  const [passErr, setPassErr] = useState("");

  const dispatch = useDispatch();
  const history = useHistory();

  useEffect(() => {
    console.log(deleteAccount);
  }, [deleteAccount]);

  const confirmDel = useCallback(async () => {
    if (pass.length === 0) {
      setPassErr("Enter your password to continue");
      return false;
    }

    await dispatch(delAccount());

    // history.push("/login");
  }, [dispatch, pass]);

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
      <Button type="critical">
        <p className="logout">Delete account</p>
      </Button>
    </InputConfirmationBox>
  );
}

export default DelAccountBtn;
