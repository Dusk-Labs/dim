import React, { Component } from "react";
import { Route, Redirect } from "react-router-dom";
import { connect } from "react-redux";

import { updateAuthToken } from "../actions/auth.js";

class PrivateRoute extends Component {
    componentDidMount() {
        const token = document.cookie.split("=")[1];

		if (token) {
			this.props.updateAuthToken(token);
		}
    }

    componentDidUpdate(prevProps) {
        if (prevProps.auth.logged_in !== this.props.auth.login.logged_in) {
            const token = document.cookie.split("=")[1];

			if (!this.props.auth.login.error && !token) {
                const dateExpires = new Date();

                dateExpires.setTime(dateExpires.getTime() + 604800000);
				document.cookie = `token=${this.props.auth.token};expires=${dateExpires.toGMTString()};`;
			}
		}
	}

    render() {
        let route;

        // LOGGED IN
		if (this.props.auth.login.logged_in && this.props.auth.token && !this.props.auth.login.error) {
            const { exact, path, render, children } = this.props;
            route = <Route exact={exact} path={path} render={render} children={children}/>;
        }

        // NOT LOGGED IN
        if (!this.props.auth.login.logged_in || !this.props.auth.token || this.props.auth.login.error) {
            route = <Redirect to="/login"/>
        }

        return route;
    }
}

const mapStateToProps = (state) => ({
    auth: state.auth
});

const mapActionsToProps = ({
    updateAuthToken
});

export default connect(mapStateToProps, mapActionsToProps)(PrivateRoute);
