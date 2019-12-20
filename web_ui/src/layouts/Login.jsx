import React, { Component } from "react";
import { connect } from "react-redux";
import { authenticate } from "../actions/authActions.js";

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
            <div className="auth-form">
                <input type="text" name="login" id="login" onChange={this.updateUsername}/>
                <input type="password" name="pass" id="pass" onChange={this.updatePassword}/>
                <button type="submit" value="ok" onClick={() => this.props.authenticate(username, password)}>Log in</button>
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
