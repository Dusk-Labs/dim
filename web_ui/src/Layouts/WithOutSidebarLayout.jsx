
import React from "react";

const WithOutSidebarLayout = (props) => (
	<main className="full">
		{props.children}
	</main>
);

export default WithOutSidebarLayout;