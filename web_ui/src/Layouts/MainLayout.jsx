import React, { Fragment } from "react";

import Sidebar from "../Components/Sidebar.jsx";

const MainLayout = (props) => (
	<Fragment>
		<Sidebar/>
		<main className="shrunk">
			{props.children}
		</main>
	</Fragment>
);

export default MainLayout;