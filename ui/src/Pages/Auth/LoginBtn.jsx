import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { authenticate } from "../../actions/auth.js";

function LoginBtn(props) {
  const dispatch = useDispatch();
  const auth = useSelector((store) => store.auth);

  const { credentials, error } = props;

  const [username, password] = credentials;
  const [setUsernameErr, setPasswordErr] = error;

  const authorize = useCallback(async () => {
    if (auth.logging_in) return;

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
    auth.logging_in,
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
    <button className={`${auth.logging_in}`} onClick={authorize}>
      Login
    </button>
  );
}

export default LoginBtn;
