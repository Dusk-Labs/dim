import React, { useCallback, useEffect } from "react";
import { connect } from "react-redux";
import { authenticate } from "../../actions/auth.js";

function LoginBtn(props) {
  const { credentials, error } = props;

  const [username, password] = credentials;
  const [setUsernameErr, setPasswordErr] = error;

  const authorize = useCallback(async () => {
    if (props.auth.logging_in) return;

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

    await props.authenticate(username, password);
  }, [credentials]);

  const onKeyDown = useCallback(e => {
    if (e.keyCode === 13) {
      authorize();
    }
  }, [credentials])

  useEffect(() => {
    window.addEventListener("keydown", onKeyDown);

    return () => {
      window.removeEventListener("keydown", onKeyDown);
    }
  }, [credentials]);

  return (
    <button className={`${props.auth.logging_in}`} onClick={authorize}>Login</button>
  )
}

const mapStateToProps = (state) => ({
  auth: state.auth.login,
});

const mapActionsToProps = {
  authenticate,
};

export default connect(mapStateToProps, mapActionsToProps)(LoginBtn);
