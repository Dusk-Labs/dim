import React, { useCallback, useEffect } from "react";
import { connect } from "react-redux";
import { register, authenticate } from "../../actions/auth.js";

function RegisterBtn(props) {
  const { credentials, error } = props;

  const [username, password1, password2, invite] = credentials;
  const [setUsernameErr, setPassword1Err, setPassword2Err, setInviteErr] = error;

  const authorize = useCallback(() => {
    if (props.registering) return;

    const allowedChars = /^[a-zA-Z0-9_.-]*$/;

    const usernameValidChars = allowedChars.test(username);
    const usernameValidLength = username.length >= 3 && username.length <= 30;

    if (!usernameValidLength) {
      setUsernameErr("Minimum 3 and maximum 30 characters");
      return;
    }

    if (!usernameValidChars) {
      setUsernameErr("Only allowed underscores, dashes or dots.");
      return;
    }

    if (password1.length < 8) {
      setPassword1Err("Minimum 8 characters.");
      return;
    }

    if (password1 !== password2) {
      setPassword2Err("Does not match");
      return;
    }

    if (!props.auth.admin_exists && invite.length !== 36) {
      setInviteErr("Code has to be 36 characters");
      return;
    }

    props.register(username, password1, invite);
    props.authenticate(username, password1);
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
  }, [credentials])

  return (
    <button className={`${props.auth.registering}`} onClick={authorize}>Login</button>
  )
}

const mapStateToProps = (state) => ({
  auth: state.auth.register,
});

const mapActionsToProps = {
  register,
  authenticate
};

export default connect(mapStateToProps, mapActionsToProps)(RegisterBtn);
