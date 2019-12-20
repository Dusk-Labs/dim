import React, { Component } from "react";
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

import { authenticate } from "./actions/authActions.js";

import './App.scss';

library.add(fas, far);

// quick hack to get proper requests
window.host = window.location.hostname;
window.host = "86.21.150.167";

class App extends Component {
    constructor(props) {
        super(props);
    }

	render() {
        if (!this.props.auth.logged_in) {
            return (
                <Login/>
            );
        }

		return (
			<Router>
			<Switch>
				<Route exact path="/" render={() => {
					document.title = "Dim - Dashboard";

					return (
						<div className="App">
							<Sidebar/>
							<main>
								<BannerPage/>
								<CardList path={`//${window.host}:8000/api/v1/dashboard`}/>
							</main>
						</div>
					);
				}}/>
				<Route exact path="/library/:id" render={props =>
					<div className="App">
						<Sidebar/>
						<main>
							<CardList path={`//${window.host}:8000/api/v1/library/${props.match.params.id}/media`}/>
						</main>
					</div>
				}/>
				<Route exact path="/search" render={props =>
					<div className="App">
						<Sidebar/>
						<main>
							<SearchResults {...props}/>
						</main>
					</div>
				}/>
				<Route exact path="/play/:id" render={props =>
					<div className="App">
						<main>
							<VideoPlayer {...props}/>
						</main>
					</div>
				}/>
                <Route exact path="/media/:id" render={props =>
					<div className="App">
					    <Sidebar/>
                        <main>
                            <MediaPage {...props}/>
                        </main>
                    </div>
                }/>
			</Switch>
			</Router>
		);
	}
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
});

const mapActionsToProps = {
    authenticate,
};

export default connect(mapStateToProps, mapActionsToProps)(App);
