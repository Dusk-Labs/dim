import React, { useCallback, useEffect } from "react";
import { connect } from "react-redux";
import { register, authenticate } from "../../actions/auth.js";

function RegisterBtn(props) {
  const { credentials, error } = props;

  const [username, pass, invite] = credentials;
  const [setUsernameErr, setPassErr, setInviteErr] = error;

  const authorize = useCallback(async () => {
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

    if (pass.length < 8) {
      setPassErr("Minimum 8 characters.");
      return;
    }

    if (props.admin_exists) {
      if (invite.length !== 36) {
        setInviteErr("Code has to be 36 characters.");
        return;
      }

      await props.register(username, pass, invite);
      props.authenticate(username, pass);
    } else {
      await props.register(username, pass);
      props.authenticate(username, pass);
    }
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
    <button className={`${props.auth.registering}`} onClick={authorize}>Register</button>
  )
}

const mapStateToProps = (state) => ({
  auth: state.auth.register,
  admin_exists: state.auth.admin_exists
});

const mapActionsToProps = {
  register,
  authenticate
};

export default connect(mapStateToProps, mapActionsToProps)(RegisterBtn);
