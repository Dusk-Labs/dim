import React, { Component } from "react";
import Sidebar from "./components/Sidebar.jsx";
import './App.css';

class App extends Component {
	render() {
		return (
			<div className="app-root">
				<Sidebar/>
			</div>
		);
	}
}

export default App;
