import { useCallback } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import ConfirmationBox from "../../Modals/ConfirmationBox.jsx";

import { logout } from "../../actions/auth.js";

function LogoutBtn() {
  const dispatch = useDispatch();
  const history = useHistory();

  const confirmLogout = useCallback(() => {
    dispatch(logout());
    history.push("/login");
  }, [dispatch, history]);

  return (
    <ConfirmationBox
      title="Confirm action"
      cancelText="Nevermind"
      confirmText="Yes"
      action={confirmLogout}
      msg="Are you sure you want to logout?"
    >
      <button className="item logout">
        <FontAwesomeIcon icon="sign-out-alt"/>
        <p className="logout">Logout</p>
      </button>
    </ConfirmationBox>
  );
}

export default LogoutBtn;
