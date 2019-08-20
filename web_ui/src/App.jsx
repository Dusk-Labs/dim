import React, { Component } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";

import Sidebar from "./layouts/Sidebar.jsx";
import Dashboard from "./layouts/Dashboard.jsx";
import Library from "./layouts/Library.jsx";

import './App.scss';

library.add(fas);

class App extends Component {
	render() {
		return (
			<Router>
				<div className="App">
					<Sidebar/>
					<Switch>
						<Route exact path="/" component={Dashboard}/>
						<Route exact path="/library/:id" component={(props) => <main><Library path={`http://86.21.150.167:8000/api/v1/library/${props.match.params.id}/media`}/></main>}/>
					</Switch>
				</div>
			</Router>
		);
	}
}

export default App;
