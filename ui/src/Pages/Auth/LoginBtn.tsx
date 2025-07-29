import React, { useCallback, useEffect } from "react";

import { useAppDispatch, useAppSelector } from "hooks/store";
import { authenticate } from "actions/auth.js";

interface Props {
  credentials: [string, string];
  error: [
    React.Dispatch<React.SetStateAction<string>>,
    React.Dispatch<React.SetStateAction<string>>
  ];
}

function LoginBtn(props: Props) {
  const dispatch = useAppDispatch();
  const auth = useAppSelector((store) => store.auth);

  const { credentials, error } = props;

  const [username, password] = credentials;
  const [setUsernameErr, setPasswordErr] = error;

  const authorize = useCallback(async () => {
    if (auth.login.logging_in) return;

    const allowedChars = /^[a-zA-Z0-9_.-]*$/;

    const usernameValidChars = allowedChars.test(username);
    const usernameValidLength = username.length >= 3 && username.length <= 30;

    if (!usernameValidLength) {
      setUsernameErr("Minimum 3 and maximum 30 characters");
      return;
    }

    if (!usernameValidChars) {
      setUsernameErr("Only allowed underscores, dashes or dots");
      return;
    }

    if (password.length < 8) {
      setPasswordErr("Minimum 8 characters");
      return;
    }

    dispatch(authenticate(username, password));
  }, [
    auth.login.logging_in,
    dispatch,
    password,
    setPasswordErr,
    setUsernameErr,
    username,
  ]);

  const onKeyDown = useCallback(
    (e) => {
      if (e.keyCode === 13) {
        authorize();
      }
    },
    [authorize]
  );

  useEffect(() => {
    window.addEventListener("keydown", onKeyDown);

    return () => {
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [onKeyDown]);

  return (
    <button className={`${auth.login.logging_in}`} onClick={authorize}>
      Login
    </button>
  );
}

export default LoginBtn;
