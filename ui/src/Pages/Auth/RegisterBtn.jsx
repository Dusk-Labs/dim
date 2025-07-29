import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { register, authenticate } from "../../actions/auth.js";

function RegisterBtn(props) {
  const dispatch = useDispatch();

  const { auth, admin_exists } = useSelector((store) => ({
    auth: store.auth,
    admin_exists: store.auth.admin_exists,
  }));

  const { credentials, error, registering } = props;

  const [username, pass, invite] = credentials;
  const [setUsernameErr, setPassErr, setInviteErr] = error;

  const authorize = useCallback(async () => {
    if (registering) return;

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

    if (admin_exists) {
      if (invite.length !== 36) {
        setInviteErr("Code has to be 36 characters.");
        return;
      }

      await dispatch(register(username, pass, invite));
      dispatch(authenticate(username, pass));
    } else {
      await dispatch(register(username, pass));
      dispatch(authenticate(username, pass));
    }
  }, [
    admin_exists,
    dispatch,
    invite,
    pass,
    registering,
    setInviteErr,
    setPassErr,
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
    <button className={`${auth.registering}`} onClick={authorize}>
      Register
    </button>
  );
}

export default RegisterBtn;
