import { useState } from "react";
import { useSelector } from "react-redux";
import Field from "../Auth/Field";

import "./Profile.scss";

function Profile() {
  const user = useSelector(store => store.user);

  const [newUsername, setNewUsername] = useState("");
  const [newUsernameErr, setNewUsernameErr] = useState("");

  return (
    <div className="preferencesProfile">
      <section>
        <h2>Avatar</h2>
        <div className="options">
          <button>
            Upload new picture
          </button>
          <button className="secondary">
            Remove avatar
          </button>
        </div>
      </section>
      <section className="usernameSection">
        <h2>Username</h2>
        <Field
          placeholder={user.username}
          data={[newUsername, setNewUsername]}
          error={[newUsernameErr, setNewUsernameErr]}
        />
        <button>Change username</button>
      </section>
    </div>
  );
}

export default Profile;
