import Sidebar from "../Components/Sidebar/Index";

const MainLayout = (props) => (
  <>
    <Sidebar />
    <main className="shrunk">{props.children}</main>
  </>
);

export default MainLayout;
