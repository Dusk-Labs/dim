import React, { Component } from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { authenticate } from "../actions/authActions.js";

import "./Login.scss";

class Login extends Component {
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
            }
        };

        this.updateField = this.updateField.bind(this);
        this.authorize = this.authorize.bind(this);
    }

    componentDidMount() {
		document.title = "Dim - Login";
    }

    componentDidUpdate(prevProps) {
        if (prevProps.auth.error !== this.props.auth.error) {
            if (this.props.auth.error === "Forbidden") {
                this.warn("password", "Wrong password");
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
        // AUTH_LOGIN_ERR
        if (this.props.auth.error) {
            console.log("[AUTH] ERROR", this.props.auth);
        }

        return (
            <div className="auth">
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
                        <input type="text" name="username" onChange={this.updateField}/>
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
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
});

const mapActionsToProps = {
    authenticate,
};

export default connect(mapStateToProps, mapActionsToProps)(Login);
