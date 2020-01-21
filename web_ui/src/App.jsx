import React, { Component, Fragment } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { connect } from "react-redux";

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import Sidebar from "./layouts/Sidebar.jsx";
import CardList from "./layouts/CardList.jsx";
import VideoPlayer from "./layouts/VideoPlayer.jsx";
import SearchResults from "./layouts/SearchResults";
import BannerPage from "./components/BannerPage.jsx";
import MediaPage from "./layouts/MediaPage.jsx";
import Login from "./layouts/Login.jsx";

import { updateAuthToken } from "./actions/authActions.js";

import './App.scss';

library.add(fas, far);

// quick hack to get proper requests
window.host = window.location.hostname;
window.host = "86.21.150.167";

class App extends Component {
    constructor(props) {
        super(props);
	}

	componentDidMount() {
		const token = document.cookie.split("=")[1];

		if (token) {
			this.props.updateAuthToken(token);
		}
	}

	dashboard() {
		document.title = "Dim - Dashboard";

		return (
			<Fragment>
				<Sidebar/>
				<main>
					<BannerPage/>
					<CardList path={`//${window.host}:8000/api/v1/dashboard`}/>
				</main>
			</Fragment>
		);
	}

	library(props) {
		return (
			<Fragment>
				<Sidebar/>
				<main>
					<CardList path={`//${window.host}:8000/api/v1/library/${props.match.params.id}/media`}/>
				</main>
			</Fragment>
		);
	}

	search(props) {
		return (
			<Fragment>
				<Sidebar/>
				<main>
					<SearchResults {...props}/>
				</main>
			</Fragment>
		);
	}

	play(props) {
		return (
			<Fragment>
				<main>
					<VideoPlayer {...props}/>
				</main>
			</Fragment>
		);
	}

	media(props) {
		return (
			<Fragment>
				<Sidebar/>
				<main>
					<MediaPage {...props}/>
				</main>
			</Fragment>
		);
	}

	componentDidUpdate(prevProps) {
		if (prevProps.auth.logged_in !== this.props.auth.logged_in) {
			const token = document.cookie.split("=")[1];

			if (!this.props.auth.error && !token) {
				const dateExpires = new Date();
				dateExpires.setTime(dateExpires.getTime() + 604800000);
				document.cookie = `token=${this.props.auth.token};expires=${dateExpires.toGMTString()};`;
			}
		}
	}

	render() {
		let app;

		if (!this.props.auth.logged_in && !this.props.auth.token || this.props.auth.error) {
            app = (
				<Fragment>
					<main>
						<Login/>
					</main>
				</Fragment>
			);
		}

		// AUTH_LOGIN_OK
		if (this.props.auth.logged_in && this.props.auth.token && !this.props.auth.error) {
			app = (
				<Switch>
					<Route exact path="/" render={this.dashboard}/>
					<Route exact path="/library/:id" render={props => this.library(props)}/>
					<Route exact path="/search" render={props => this.search(props)}/>
					<Route exact path="/play/:id" render={props => this.play(props)}/>
					<Route exact path="/media/:id" render={props => this.media(props)}/>
				</Switch>
			);
		}

		return (
			<Router>
				<div className="App">
					{app}
				</div>
			</Router>
		);
	}
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
});

const mapActionsToProps = ({
    updateAuthToken,
});

export default connect(mapStateToProps, mapActionsToProps)(App);
