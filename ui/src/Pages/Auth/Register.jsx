import React, { useEffect, useState } from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";

import { fetchUser } from "../../actions/user.js";
import { authenticate, register, checkAdminExists } from "../../actions/auth.js";
import RegisterBtn from "./RegisterBtn";
import Field from "./Field";
import DimLogo from "../../assets/DimLogo";

import "./AuthForm.scss";

function Register(props) {
  const [username, setUsername] = useState("");
  const [usernameErr, setUsernameErr] = useState("");

  const [pass, setPass] = useState("");
  const [passErr, setPassErr] = useState("");

  const [invite, setInvite] = useState("");
  const [inviteErr, setInviteErr] = useState("");

  const { checkAdminExists, auth } = props;

  // AUTH_LOGIN_ERR
  useEffect(() => {
    if (auth.register.error) {
      setInviteErr(auth.register.error);
    }
  }, [auth.register.error]);

  useEffect(() => { checkAdminExists() }, [checkAdminExists]);

  return (
    <div className="authForm">
      <header>
        <DimLogo/>
        <h1>Welcome to Dim</h1>
        {props.auth.admin_exists
          ? <h3>A media manager fueled by dark forces</h3>
          : <h3>You are making an admin account</h3>
        }
      </header>
      <div className="fields">
        <Field
          name="Username"
          icon="user"
          data={[username, setUsername]}
          error={[usernameErr, setUsernameErr]}
        />
        <Field
          name="Password"
          icon="key"
          data={[pass, setPass]}
          error={[passErr, setPassErr]}
          type="password"
        />
        {props.auth.admin_exists && (
          <Field
            name="Invite token"
            icon="key"
            data={[invite, setInvite]}
            error={[inviteErr, setInviteErr]}
          />
        )}
      </div>
      <footer>
        <RegisterBtn
          credentials={[username, pass, invite]}
          error={[setUsernameErr, setPassErr, setInviteErr]}
        />
        <Link to="/login">I have an account</Link>
      </footer>
    </div>
  );
}

const mapStateToProps = (state) => ({
  auth: state.auth,
});

const mapActionsToProps = {
  authenticate,
  register,
  checkAdminExists,
  fetchUser
};

export default connect(mapStateToProps, mapActionsToProps)(Register);
