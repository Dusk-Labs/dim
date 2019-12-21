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

import './App.scss';

library.add(fas, far);

// quick hack to get proper requests
window.host = window.location.hostname;
window.host = "86.21.150.167";

class App extends Component {
    constructor(props) {
        super(props);
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

	render() {
		let app;

		if (!this.props.auth.logged_in && !this.props.auth.token || this.props.auth.error) {
            app = <Login/>;
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

export default connect(mapStateToProps)(App);
