import ManagePassword from "./ManagePassword";
import ManageAccount from "./ManageAccount";

import "./Index.scss";

const Account = () => (
  <div className="preferencesAccount">
    <ManagePassword />
    <ManageAccount />
  </div>
);

export default Account;
