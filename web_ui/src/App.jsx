import React, { Component } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import Main from "./components/Main.jsx";
import Sidebar from "./components/Sidebar.jsx";
import './App.scss';

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";

library.add(fas);

class App extends Component {
	render() {
		return (
			<Router>
				<div className="App">
					<Sidebar/>
					<Main/>
				</div>
			</Router>
		);
	}
}

export default App;
