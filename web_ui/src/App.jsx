import React, { Component, Fragment } from "react";
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

class App extends Component {
	constructor(props) {
		super(props);

		this.main = React.createRef();
	}

	render() {
		return (
			<Router>
			<div className="App">
				<Switch>
					<Route exact path="/" render={_ => {
						document.title = "Dim - Dashboard";

						return (
							<Fragment>
								<Sidebar main={this.main}/>
								<main ref={this.main}>
									<BannerPage/>
									<CardList path="http://127.0.0.1:8000/api/v1/dashboard"/>
								</main>
							</Fragment>
						);
					}}/>

					<Route exact path="/library/:id" render={props =>
						<Fragment>
							<Sidebar main={this.main}/>
							<main ref={this.main}>
								<CardList path={`http://127.0.0.1:8000/api/v1/library/${props.match.params.id}/media`}/>
							</main>
						</Fragment>
					}/>

					<Route exact path="/search" render={props =>
						<Fragment>
							<Sidebar main={this.main}/>
							<main ref={this.main}>
								<SearchResults {...props}/>
							</main>
						</Fragment>
					}/>

					<Route exact path="/play/:id" render={props =>
						<main>
							<VideoPlayer {...props}/>
						</main>
					}/>
				</Switch>
			</div>
			</Router>
		);
	}
}

export default App;
