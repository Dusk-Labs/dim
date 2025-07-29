import General from "./General";
import DirectoryPaths from "./DirectoryPaths";
import Authentication from "./Authentication";

import "./Index.scss";

const PreferencesAdvanced = () => (
  <div className="preferencesAdvanced">
    <General />
    <DirectoryPaths />
    <Authentication />
  </div>
);

export default PreferencesAdvanced;
