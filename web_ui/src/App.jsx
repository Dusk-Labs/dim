import React, { Component } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import Sidebar from "./layouts/Sidebar.jsx";
import CardList from "./layouts/CardList.jsx";
import VideoPlayer from "./layouts/VideoPlayer.jsx";
import SearchResults from "./layouts/SearchResults";
import BannerPage from "./components/BannerPage.jsx";

import './App.scss';

library.add(fas, far);

window.host = window.location.hostname; // quick hack to get proper requests

class App extends Component {
	render() {
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
								<CardList path={`http://${window.host}:8000/api/v1/dashboard`}/>
							</main>
						</div>
					);
				}}/>
				<Route exact path="/library/:id" render={props =>
					<div className="App">
						<Sidebar/>
						<main>
							<CardList path={`http://${window.host}:8000/api/v1/library/${props.match.params.id}/media`}/>
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
			</Switch>
			</Router>
		);
	}
}

export default App;
