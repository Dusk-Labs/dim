import React, { useCallback, useState, useEffect } from "react";
import { connect } from "react-redux";
import { Link, useHistory } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchUser } from "../actions/user.js";
import { authenticate, updateAuthToken } from "../actions/auth.js";

import "./AuthForm.scss";

function Login(props) {
  const history = useHistory();

  const [username, setUsername] = useState("");
  const [usernameErr, setUsernameErr] = useState("");

  const [password, setPassword] = useState("");
  const [passwordErr, setPasswordErr] = useState("");

  // CHECK AUTH STATE
  useEffect(() => {
    const token = document.cookie.split("=")[1];

		if (props.auth.login.logged_in && props.auth.token && !props.auth.login.error || token) {
      props.fetchUser(props.auth.token);

      if (!token) {
        const dateExpires = new Date();

        dateExpires.setTime(dateExpires.getTime() + 604800000);

        document.cookie = (
          `token=${props.auth.token};expires=${dateExpires.toGMTString()};`
        );
      }

      return history.push("/");
    }

    // AUTH_LOGIN_ERR
    if (props.auth.login.error) {
      setPasswordErr(props.auth.login.error)
    }
  }, [props.auth]);

  useEffect(() => setUsernameErr(""), [username])
  useEffect(() => setPasswordErr(""), [password])

  const authorize = useCallback(async () => {
    if (username.length <= 3 || password.length <= 3) {
      if (username.length <= 3) {
        setUsernameErr("Too short, min. 4 chars.");
      }

      if (password.length <= 3) {
        setPasswordErr("Too short, min. 4 chars.");
      }
    } else {
      await props.authenticate(username, password);
    }
  }, [username, password]);

  return (
    <div className="authForm">
      <header>
        <h1>Welcome back</h1>
        <h3>Authenticate and continue to your media</h3>
      </header>
      <div className="fields">
        <label className="username">
          <div className="name">
            <FontAwesomeIcon icon="user"/>
            <p>Username</p>
            {usernameErr.length > 0 &&
              <div className="horizontal-err">
                <FontAwesomeIcon icon="times-circle"/>
                <p>{usernameErr}</p>
              </div>
            }
          </div>
          <input
            name="username"
            onChange={e => setUsername(e.target.value)}
            spellCheck="false"
            autocomplete="off"
            autocorrect="off"
            autocapitalize="none"
          />
        </label>
        <label className="password">
          <div className="name">
            <FontAwesomeIcon icon="key"/>
            <p>Password</p>
            {passwordErr.length > 0 &&
              <div className="horizontal-err">
                <FontAwesomeIcon icon="times-circle"/>
                <p>{passwordErr}</p>
              </div>
            }
          </div>
          <input
            type="password"
            name="password"
            onChange={e => setPassword(e.target.value)}
            autocomplete="off"
            autocorrect="off"
            autocapitalize="none"
          />
        </label>
      </div>
      <footer>
        <button onClick={authorize}>Login</button>
        <div className="actions">
          <Link to="/register">Create a new account</Link>
          <Link to="/forgot-password">Forgot password</Link>
        </div>
      </footer>
      <svg className="logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 202.832 143.542">
        <g transform="translate(-397.21 -251.229)">
          <path d="M6712.87,5823.906l34.876,34.876a9.96,9.96,0,0,1,0,14.063l-34.876,34.9a9.989,9.989,0,0,1-14.088,0l-34.876-34.9a9.96,9.96,0,0,1,0-14.063l34.876-34.876a9.989,9.989,0,0,1,14.088,0Zm33.989,35.763-34.876-34.876a8.741,8.741,0,0,0-12.314,0l-34.876,34.876a8.741,8.741,0,0,0,0,12.314l34.876,34.876a8.741,8.741,0,0,0,12.314,0l34.876-34.876a8.741,8.741,0,0,0,0-12.314Z" transform="translate(-6205.073 -5569.771)" fill="#fff"/>
          <path d="M8974.543,8036.733l34.875,34.876a9.35,9.35,0,0,1,0,13.177l-13.743,13.768V8082.89l-3.694-5.616v24.975l-17.437,17.438a9.381,9.381,0,0,1-13.2,0l-13.743-13.768v-28.645l-3.694,5.616v19.359l-17.438-17.462a9.352,9.352,0,0,1,0-13.177l34.876-34.876A9.337,9.337,0,0,1,8974.543,8036.733Z" transform="translate(-8412.092 -7728.265)" fill="#fff" fill-rule="evenodd"/>
          <path d="M4328.87,8011.906l34.875,34.876a9.959,9.959,0,0,1,0,14.063l-34.875,34.9a9.989,9.989,0,0,1-14.088,0l-34.876-34.9a9.96,9.96,0,0,1,0-14.063l34.876-34.876a9.989,9.989,0,0,1,14.088,0Zm-9.95,85v-86.154a8.858,8.858,0,0,0-3.251,2.02l-34.875,34.9a8.71,8.71,0,0,0,0,12.29l34.875,34.9A8.946,8.946,0,0,0,4318.92,8096.9Z" transform="translate(-3879.79 -7703.881)" fill="#fff"/>
          <rect width="14" height="88" rx="7" transform="translate(494 252)" fill="#fff"/>
        </g>
      </svg>
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
