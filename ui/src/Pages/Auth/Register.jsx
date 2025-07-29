import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Link, useLocation } from "react-router-dom";

import { checkAdminExists } from "../../actions/auth.js";
import RegisterBtn from "./RegisterBtn";
import Field from "./Field";
import DimLogo from "../../assets/DimLogo";

import "./AuthForm.scss";

function useQuery() {
  const { search } = useLocation();

  return React.useMemo(() => new URLSearchParams(search), [search]);
}

function Register(props) {
  let token = useQuery()?.get("token");

  const dispatch = useDispatch();
  const auth = useSelector((store) => store.auth);

  const [username, setUsername] = useState("");
  const [usernameErr, setUsernameErr] = useState("");

  const [pass, setPass] = useState("");
  const [passErr, setPassErr] = useState("");

  const [invite, setInvite] = useState("");
  const [inviteErr, setInviteErr] = useState("");

  // AUTH_LOGIN_ERR
  useEffect(() => {
    if (auth.register.error) {
      setInviteErr(auth.register.error);
    }
  }, [auth.register.error]);

  useEffect(() => {
    dispatch(checkAdminExists());
  }, [dispatch]);

  useEffect(() => {
    if (token) setInvite(token);
  }, [setInvite, token]);

  return (
    <div className="authForm">
      <header>
        <DimLogo />
        <h1>Welcome to Dim</h1>
        {auth.admin_exists ? (
          <h3>A media manager fueled by dark forces</h3>
        ) : (
          <h3>You are making an admin account</h3>
        )}
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
        {auth.admin_exists && (
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
        {auth.admin_exists && <Link to="/login">I have an account</Link>}
      </footer>
    </div>
  );
}

export default Register;
