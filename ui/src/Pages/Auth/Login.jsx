import { useState, useEffect } from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";

import { fetchUser } from "../../actions/user.js";
import { authenticate, updateAuthToken } from "../../actions/auth.js";
import DimLogo from "../../assets/DimLogo";
import Field from "./Field";
import LoginBtn from "./LoginBtn";

import "./AuthForm.scss";

function Login(props) {
  const [username, setUsername] = useState("");
  const [usernameErr, setUsernameErr] = useState("");

  const [password, setPassword] = useState("");
  const [passwordErr, setPasswordErr] = useState("");

  // AUTH_LOGIN_ERR
  useEffect(() => {
    if (props.auth.login.error) {
      setPasswordErr(props.auth.login.error)
    }
  }, [props.auth]);

  return (
    <div className="authForm">
      <header>
        <DimLogo/>
        <h1>Welcome back</h1>
        <h3>Authenticate and continue to your media</h3>
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
          data={[password, setPassword]}
          error={[passwordErr, setPasswordErr]}
          type="password"
        />
      </div>
      <footer>
        <LoginBtn
          credentials={[username, password]}
          error={[setUsernameErr, setPasswordErr]}
        />
        <div className="actions">
          <Link to="/register">Create a new account</Link>
        </div>
      </footer>
    </div>
  )
}

const mapStateToProps = (state) => ({
  auth: state.auth,
});

const mapActionsToProps = {
  authenticate,
  updateAuthToken,
  fetchUser
};

export default connect(mapStateToProps, mapActionsToProps)(Login);
