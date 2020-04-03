import React, { Component } from "react";
import { connect } from "react-redux";
import { Link, Redirect } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchUser } from "../actions/user.js";
import { authenticate, updateAuthToken } from "../actions/auth.js";

import WithOutSidebarLayout from "../Layouts/WithOutSidebarLayout.jsx";

import "./AuthForm.scss";

class Login extends Component {
    constructor(props) {
        super(props);

        this.updateField = this.updateField.bind(this);
        this.authorize = this.authorize.bind(this);
        this.onKeyDown = this.onKeyDown.bind(this);

        this.state = {
            username: {
                value: "",
                err: ""
            },
            password: {
                value: "",
                err: ""
            }
        };
    }

    componentDidMount() {
        document.title = "Dim - Login";

        document.addEventListener("keydown", this.onKeyDown);
    }

    componentWillUnmount() {
        document.removeEventListener("keydown", this.onKeyDown);
    }

    componentDidUpdate(prevProps) {
        if (prevProps.auth.login.error !== this.props.auth.login.error) {
            if (this.props.auth.login.error === "Forbidden") {
                this.warn("password", "Wrong password");
            }
        }
    }

    onKeyDown(e) {
        if (e.keyCode !== 13) return;
        this.authorize();
    }

    updateField(e) {
        const { name, value } = e.target;

        this.setState({
            [name]: {
                value,
                err: ""
            }
        });
    }

    warn(field, err) {
        this.setState({
            [field]: {
                ...this.state[field],
                err
            }
        });
    }

    authorize() {
        const { username, password } = this.state;

        if (username.value.length <= 3 || password.value.length <= 3) {
            if (username.value.length <= 3) {
                this.warn("username", "Too short, min. 4 chars.");
            }

            if (password.value.length <= 3) {
                this.warn("password", "Too short, min. 4 chars.");
            }
        } else {
            this.props.authenticate(username.value, password.value);
        }
    }

    render() {
        const token = document.cookie.split("=")[1];

        // LOGGED IN
		if (this.props.auth.login.logged_in && this.props.auth.token && !this.props.auth.login.error || token) {
            this.props.fetchUser(this.props.auth.token);

            if (!token) {
                const dateExpires = new Date();

                dateExpires.setTime(dateExpires.getTime() + 604800000);
                document.cookie = `token=${this.props.auth.token};expires=${dateExpires.toGMTString()};`;
            }

            return <Redirect to="/"/>;
        }

        // AUTH_LOGIN_ERR
        if (this.props.auth.login.error) {
            console.log("[AUTH] ERROR", this.props.auth.login);
        }

        const loginForm = (
            <div className="auth-form">
                <header>
                    <h1>Welcome to Dim</h1>
                    <h3>A media manager fueled by dark forces</h3>
                </header>
                <div className="fields">
                    <div className="field">
                        <label>
                            <FontAwesomeIcon icon="user"/>
                            <p>USERNAME</p>
                            {this.state.username.err.length > 0 &&
                                <div className="horizontal-err">
                                    <FontAwesomeIcon icon="times-circle"/>
                                    <p>{this.state.username.err}</p>
                                </div>
                            }
                        </label>
                        <input type="text" name="username" onChange={this.updateField} spellCheck="false" autoFocus/>
                    </div>
                    <div className="field">
                        <label>
                            <FontAwesomeIcon icon="key"/>
                            <p>PASSWORD</p>
                            {this.state.password.err.length > 0 &&
                                <div className="horizontal-err">
                                    <FontAwesomeIcon icon="times-circle"/>
                                    <p>{this.state.password.err}</p>
                                </div>
                            }
                        </label>
                        <input type="password" name="password" onChange={this.updateField}/>
                    </div>
                </div>
                <footer>
                    <button onClick={this.authorize}>Login</button>
                    <Link to="/register">Create an account</Link>
                </footer>
            </div>
        );

        return (
            <WithOutSidebarLayout>
                {loginForm}
            </WithOutSidebarLayout>
        )
    }
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
