import { useState, useEffect } from "react";
import { useSelector } from "react-redux";
import { Link } from "react-router-dom";

import DimLogo from "../../assets/DimLogo";
import Field from "./Field";
import LoginBtn from "./LoginBtn";

import "./AuthForm.scss";

function Login() {
  const auth = useSelector((store) => store.auth);

  const [username, setUsername] = useState("");
  const [usernameErr, setUsernameErr] = useState("");

  const [password, setPassword] = useState("");
  const [passwordErr, setPasswordErr] = useState("");

  // AUTH_LOGIN_ERR
  useEffect(() => {
    if (auth.login.error) {
      setPasswordErr(auth.login.error);
    }
  }, [auth.login.error]);

  return (
    <div className="authForm">
      <header>
        <DimLogo />
        <h1>Welcome back</h1>
        <h3>Authenticate and continue to your media</h3>
      </header>
      <div className="fields">
        <Field
          name="Username"
          icon="user"
          data={[username, setUsername]}
          error={[usernameErr, setUsernameErr]}
          autocomplete="username"
        />
        <Field
          name="Password"
          icon="key"
          data={[password, setPassword]}
          error={[passwordErr, setPasswordErr]}
          type="password"
          autocomplete="current-password"
        />
      </div>
      <footer>
        <LoginBtn
          credentials={[username, password]}
          error={[setUsernameErr, setPasswordErr]}
        />
        <Link to="/register">Create a new account</Link>
      </footer>
    </div>
  );
}

export default Login;
