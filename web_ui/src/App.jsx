import React, { Component } from "react";
import Main from "./components/Main.jsx";
import Sidebar from "./components/Sidebar.jsx";
import './App.scss';

import { library } from "@fortawesome/fontawesome-svg-core";
import { fas } from "@fortawesome/free-solid-svg-icons";

library.add(fas);

class App extends Component {
	render() {
		return (
			<div className="App">
				<Sidebar/>
				<Main/>
			</div>
		);
	}
}

export default App;
