import React, { Component } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { connect } from "react-redux";

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import PrivateRoute from "./Routes/PrivateRoute.jsx";

import Dashboard from "./Pages/Dashboard.jsx";
import Library from "./Pages/Library.jsx";
import Media from "./Pages/Media.jsx";
// import VideoPlayer from "./Pages/VideoPlayer.jsx";
import SearchResults from "./Pages/SearchResults.jsx";
import Login from "./Pages/Login/Index.jsx";
import Register from "./Pages/Register.jsx";
import Preferences from "./Pages/Preferences.jsx";

import { updateAuthToken } from "./actions/auth.js";

import "./App.scss";

import MainLayout from "./Layouts/MainLayout.jsx";
import WithOutSidebarLayout from "./Layouts/WithOutSidebarLayout.jsx";

library.add(fas, far);

// quick hack to get proper requests
window.host = window.location.hostname;
// window.host = "86.21.150.167";

const routes = (
	<Switch>
		<Route exact path="/login">
			<Login/>
		</Route>
		<Route exact path="/register">
			<WithOutSidebarLayout>
				<Register/>
			</WithOutSidebarLayout>
		</Route>
		<PrivateRoute exact path="/">
			<MainLayout>
				<Dashboard/>
			</MainLayout>
		</PrivateRoute>
		<PrivateRoute exact path="/library/:id">
			<MainLayout>
				<Library/>
			</MainLayout>
		</PrivateRoute>
		<PrivateRoute exact path="/preferences">
			<MainLayout>
				<Preferences/>
			</MainLayout>
		</PrivateRoute>
		<PrivateRoute path="/search" render={(props) => (
			<MainLayout>
				<SearchResults {...props}/>
			</MainLayout>
		)}/>
		<PrivateRoute path="/media/:id" render={(props) => (
			<MainLayout>
				<Media {...props}/>
			</MainLayout>
		)}/>
		{/* <PrivateRoute path="/play/:id" render={(props) => (
			<WithOutSidebarLayout>
				<VideoPlayer {...props}/>
			</WithOutSidebarLayout>
		)}/> */}
	</Switch>
);

class App extends Component {
	componentDidMount() {
		const darkLogo = document.getElementById("logo-dark");
		const lightLogo = document.getElementById("logo-light");

		if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
			lightLogo.remove();
		}

		window.matchMedia("(prefers-color-scheme: dark)").addListener((e) => {
			if (e.matches) {
				lightLogo.remove();
				document.head.append(darkLogo);
			} else {
				darkLogo.remove();
				document.head.append(lightLogo);
			}
		});
	}

	render() {
		return (
			<Router>
				{routes}
			</Router>
		);
	}
}

const mapStateToProps = (state) => ({
    auth: state.authReducer
});

const mapActionsToProps = ({
    updateAuthToken
});

export default connect(mapStateToProps, mapActionsToProps)(App);
