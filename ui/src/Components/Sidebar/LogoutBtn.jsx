import { useCallback } from "react";
import { useHistory } from "react-router-dom";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import ConfirmationBox from "../../Modals/ConfirmationBox.jsx";

import { logout } from "../../actions/auth.js";

function LogoutBtn(props) {
  const history = useHistory();

  const { logout } = props;

  const confirmLogout = useCallback(() => {
    logout();
    history.push("/login");
  }, [history, logout]);

  return (
    <ConfirmationBox
      contentLabel="removeLib"
      action={confirmLogout}
      msg="Are you sure you want to logout?"
    >
      <button className="item logout">
        <FontAwesomeIcon icon="sign-out-alt"/>
        <p className="logout">Logout</p>
      </button>
    </ConfirmationBox>
  );
};

const mapStateToProps = () => ({});

const mapActionsToProps = {
  logout
};

export default connect(mapStateToProps, mapActionsToProps)(LogoutBtn);
