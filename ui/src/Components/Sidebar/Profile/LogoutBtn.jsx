import { useCallback } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch } from "react-redux";

import ConfirmationBox from "../../../Modals/ConfirmationBox";
import Logout from "../../../assets/Icons/Logout";
import { logout } from "../../../actions/auth.js";

import "./LogoutBtn.scss";

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
      <button className="logout">
        <div className={"buttonLabel"}>
          <p>Logout</p>
        </div>
        <Logout />
      </button>
    </ConfirmationBox>
  );
}

export default LogoutBtn;
