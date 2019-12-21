import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { authenticate } from "../actions/authActions.js";

import "./Login.scss";

class Login extends Component {
    constructor(props) {
        super(props);

        this.state = {
            username: "",
            password: "",
        };

        this.updateUsername = this.updateUsername.bind(this);
        this.updatePassword = this.updatePassword.bind(this);
    }

    updateUsername(e) {
        this.setState({
            username: e.target.value,
        });
    }

    updatePassword(e) {
        this.setState({
            password: e.target.value,
        });
    }

    render() {
        const { username, password } = this.state;

        return (
            <div className="auth">
                <header>
                    <h1>Welcome to Dim</h1>
                    <h3>A media manager fueled by dark forces</h3>
                </header>
                <form>
                    <div className="field">
                        <label>
                            <FontAwesomeIcon icon="user"/>
                            <p>USERNAME</p>
                        </label>
                        <input type="text" name="login" id="login" onChange={this.updateUsername}/>
                    </div>
                    <div className="field">
                        <label>
                            <FontAwesomeIcon icon="key"/>
                            <p>PASSWORD</p>
                        </label>
                        <input type="password" name="pass" id="pass" onChange={this.updatePassword}/>
                    </div>
                </form>
                <button type="submit" value="ok" onClick={() => this.props.authenticate(username, password)}>Login</button>
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
