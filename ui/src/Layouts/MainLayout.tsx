import type { PropsWithChildren } from "react";

import Sidebar from "../Components/Sidebar/Index";

const MainLayout = (props: PropsWithChildren<{}>) => (
  <>
    <Sidebar />
    <main className="shrunk">{props.children}</main>
  </>
);

export default MainLayout;
