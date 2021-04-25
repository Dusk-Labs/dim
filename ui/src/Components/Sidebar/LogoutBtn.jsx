import { useCallback } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch } from "react-redux";

import ConfirmationBox from "../../Modals/ConfirmationBox.jsx";
import { logout } from "../../actions/auth.js";
import LogoutIcon from "../../assets/Icons/Logout";

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
        <LogoutIcon/>
        <p className="logout">Logout</p>
      </button>
    </ConfirmationBox>
  );
}

export default LogoutBtn;
