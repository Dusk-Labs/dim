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

  const [password1, setPassword1] = useState("");
  const [passwordErr1, setPassword1Err] = useState("");
  const [password2, setPassword2] = useState("");
  const [passwordErr2, setPassword2Err] = useState("");

  const [invite, setInvite] = useState("");
  const [inviteErr, setInviteErr] = useState("");

  // AUTH_LOGIN_ERR
  useEffect(() => {
      if (props.auth.register.error) {
      setInviteErr(props.auth.register.error)
    }
  }, [props.auth]);

  useEffect(() => props.checkAdminExists, []);

  return (
    <div className="authForm">
      <header>
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
          data={[password1, setPassword1]}
          error={[passwordErr1, setPassword1Err]}
          type="password"
          />
        <Field
          name="Confirm your password"
          icon="key"
          data={[password2, setPassword2]}
          error={[passwordErr2, setPassword2Err]}
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
          credentials={[username, password1, password2, invite]}
          error={[setUsernameErr, setPassword1Err, setPassword2Err, setInviteErr]}
        />
        <Link to="/login">I have an account</Link>
      </footer>
      <DimLogo/>
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
