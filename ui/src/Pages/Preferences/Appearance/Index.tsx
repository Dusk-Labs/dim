import Themes from "./Themes";
import Cards from "./Cards";
// import Sidebar from "./Sidebar";

import "./Index.scss";

function Appearance() {
  return (
    <div className="preferencesAppearance">
      <Themes />
      <Cards />
      {/* <Sidebar/> */}
    </div>
  );
}

export default Appearance;
