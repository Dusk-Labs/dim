import React, { Component } from "react";
import { connect } from "react-redux";
import { Link, Redirect } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { authenticate, register, checkAdminExists } from "../actions/authActions.js";

import "./AuthForm.scss";

class Register extends Component {
    constructor(props) {
        super(props);

        this.state = {
            username: {
                value: "",
                err: ""
            },
            password: {
                value: "",
                err: ""
            },
            confirm_password: {
                value: "",
                err: ""
            },
            invite: {
                value: "",
                err: ""
            },
        };

        this.updateField = this.updateField.bind(this);
        this.authorize = this.authorize.bind(this);
    }

    componentDidMount() {
		document.title = "Dim - Register";
        this.props.checkAdminExists();
    }

    componentDidUpdate(prevProps) {
        if (prevProps.auth.error !== this.props.auth.error) {
            if (this.props.auth.error === "NoTokenError") {
                this.warn("invite", "Wrong invite token");
            }
            if (this.props.auth.error === "UsernameTaken") {
                this.warn("username", "Username is already taken");
            }
        }
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

    async authorize() {
        const { username, password, confirm_password, invite } = this.state;

        if (password.value !== confirm_password.value )
            this.warn("confirm_password", "Passwords must match");

        if (username.value.length <= 3 || password.value.length <= 3 || (this.props.auth.admin_exists && invite.value.length !== 36)) {
            if (username.value.length <= 3) {
                this.warn("username", "Too short, min. 4 chars.");
            }

            if (password.value.length <= 3) {
                this.warn("password", "Too short, min. 4 chars.");
            }

            if (this.props.auth.admin_exists && invite.value.length !== 36) {
                this.warn("invite", "Should be 36 chars.");
            }
        } else {
            // FIXME: Track down why token remains as a null after logout
            document.cookie = "token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
            await this.props.register(username.value, password.value, invite.value);
            await this.props.authenticate(username.value, password.value);
        }
    }

    render() {
        const token = document.cookie.split("=")[1];

        // LOGGED IN
		if (this.props.auth.logged_in && this.props.auth.token && !this.props.auth.error || token) {
            if (!token) {
                const dateExpires = new Date();

                dateExpires.setTime(dateExpires.getTime() + 604800000);
                document.cookie = `token=${this.props.auth.token};expires=${dateExpires.toGMTString()};`;
            }

            return <Redirect to="/"/>;
        }

        const { admin_exists } = this.props.auth;

        // AUTH_LOGIN_ERR
        if (this.props.auth.error) {
            console.log("[AUTH] REGISTER ERROR", this.props.auth);
        }

        return (
            <div className="auth-form">
                <header>
                    <h1>Welcome to Dim</h1>
                    {admin_exists
                        ? <h3>A media manager fueled by dark forces</h3>
                        : <h3>Warning: You are making a admin account! </h3>
                    }
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
                        <input type="text" name="username" onChange={this.updateField} spellCheck="false"/>
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
                    <div className="field">
                        <label>
                            <FontAwesomeIcon icon="key"/>
                            <p>CONFIRM YOUR PASSWORD</p>
                            {this.state.confirm_password.err.length > 0 &&
                                <div className="horizontal-err">
                                    <FontAwesomeIcon icon="times-circle"/>
                                    <p>{this.state.confirm_password.err}</p>
                                </div>
                            }
                        </label>
                        <input type="password" name="confirm_password" onChange={this.updateField}/>
                    </div>
                    {admin_exists ?
                    <div className="field">
                        <label>
                            <FontAwesomeIcon icon="tag"/>
                            <p>INVITE TOKEN</p>
                            {this.state.invite.err.length > 0 &&
                                <div className="horizontal-err">
                                    <FontAwesomeIcon icon="times-circle"/>
                                    <p>{this.state.invite.err}</p>
                                </div>
                            }
                        </label>
                        <input type="invite" name="invite" onChange={this.updateField}/>
                    </div> : <div/>
                    }
                </div>
                <footer>
                    <button onClick={this.authorize}>Register</button>
                    <Link to="/login">Already have an account?</Link>
                </footer>
            </div>
        );
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
});

const mapActionsToProps = {
    authenticate, register, checkAdminExists
};

export default connect(mapStateToProps, mapActionsToProps)(Register);
