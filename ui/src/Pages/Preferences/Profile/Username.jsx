import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { changeUsername } from "../../../actions/user";
import Button from "../../../Components/Misc/Button";
import Field from "../../Auth/Field";

function Username() {
  const dispatch = useDispatch();
  const user = useSelector((store) => store.user);

  const [newUsername, setNewUsername] = useState("");
  const [newUsernameErr, setNewUsernameErr] = useState("");

  useEffect(() => {
    if (user.info.username) {
      setNewUsername(user.info.username);
    }
  }, [user.info.username]);

  const updateUsername = useCallback(() => {
    if (newUsername.length === 0) {
      setNewUsernameErr("Your new name has to be at least 1 character long.");
      return;
    }

    if (newUsername === user.info.username) {
      setNewUsernameErr("That is your current username already.");
      return;
    }

    dispatch(changeUsername(user, newUsername));
  }, [dispatch, newUsername, user]);

  const undoChangeUsername = useCallback(() => {
    setNewUsername(user.info.username);
    setNewUsernameErr("");
  }, [user.info.username]);

  return (
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
          <Button onClick={updateUsername}>Update</Button>
          <Button type="secondary" onClick={undoChangeUsername}>
            Cancel
          </Button>
        </div>
      )}
    </section>
  );
}

export default Username;
