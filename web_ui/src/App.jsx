import React, { Component } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { far } from "@fortawesome/free-regular-svg-icons";

import Sidebar from "./layouts/Sidebar.jsx";
import Dashboard from "./layouts/Dashboard.jsx";
import Library from "./layouts/Library.jsx";
import Play from "./layouts/Play.jsx";
import SearchResults from "./layouts/SearchResults";

import './App.scss';

library.add(fas, far);

class App extends Component {
	render() {
		return (
			<Router>
				<Switch>
					<Route exact path="/" render={() =>
						<div className="App">
							<Sidebar/>
							<Dashboard/>
						</div>
					}/>
					<Route exact path="/library/:id" render={props =>
						<div className="App">
							<Sidebar/>
							<main>
                                <Library path={`http://86.21.150.167:8000/api/v1/library/${props.match.params.id}/media`}
                                    id={props.match.params.id}/>
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
							<Play {...props}/>
						</div>
					}/>
				</Switch>
			</Router>
		);
	}
}

export default App;
