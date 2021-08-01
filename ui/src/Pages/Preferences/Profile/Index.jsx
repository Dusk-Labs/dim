import { useCallback, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import Button from "../../../Components/Misc/Button";
import Field from "../../Auth/Field";

import "./Index.scss";

function Profile() {
  const user = useSelector(store => store.user);

  const [newAvatar, setNewAvatar] = useState("");
  const [newUsername, setNewUsername] = useState("");
  const [newUsernameErr, setNewUsernameErr] = useState("");

  useEffect(() => {
    if (user.info.username) {
      setNewUsername(user.info.username);
    }
  }, [user.info.username]);

  const changeUsername = useCallback(() => {
    if (newUsername.length === 0) {
      setNewUsernameErr("Your new name has to be at least 1 character long.");
    }
    if (newUsername === user.info.username) {
      setNewUsernameErr("That is your current username already.");
    }
  }, [newUsername, user.info.username]);

  const clearNewAvatarUpload = useCallback(() => {
    setNewAvatar("");
  }, []);

  const uploadNewPic = useCallback(() => {
    setNewAvatar("");

    const input = document.createElement("input");

    input.type = "file";
    input.accept = "image/png, image/jpeg";

    input.addEventListener("change", (e) => {
      if (!input.files[0]) return;

      setNewAvatar(
        URL.createObjectURL(input.files[0])
      );
    });

    input.click();
  }, []);

  const undoChangeUsername = useCallback(() => {
    setNewUsername(user.info.username);
    setNewUsernameErr("");
  }, [user.info.username]);

  return (
    <div className="preferencesProfile">
      <section>
        <h2>Avatar</h2>
        {newAvatar && <img src={newAvatar} alt="New avatar"/>}
        <div className="options">
          {newAvatar && (
            <Button>
              Save as new avatar
            </Button>
          )}
          {!newAvatar && (
            <Button onClick={uploadNewPic}>
              Upload {user.info.picture ? "new" : ""} picture
            </Button>
          )}
          {newAvatar && (
            <Button type="secondary" onClick={clearNewAvatarUpload}>
              Clear upload
            </Button>
          )}
          {(!newAvatar && user.info.picture) && (
            <Button type="secondary">
              Remove current avatar
            </Button>
          )}
        </div>
      </section>
      <section className="usernameSection">
        <h2>Username</h2>
        <Field
          name="New username"
          icon="user"
          data={[newUsername, setNewUsername]}
          error={[newUsernameErr, setNewUsernameErr]}
        />
        {user.info.username !== newUsername && (
          <div className="options">
            <Button onClick={changeUsername}>
              Update
            </Button>
            <Button type="secondary" onClick={undoChangeUsername}>
              Undo
            </Button>
          </div>
        )}
      </section>
    </div>
  );
}

export default Profile;
