import React, { Component } from "react";
import Main from "./components/Main.jsx";
import Sidebar from "./components/Sidebar.jsx";
import './App.css';

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
