import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { changeAvatar, delAvatar, fetchUser } from "../../../actions/user";
import Button from "../../../Components/Misc/Button";

function Avatar() {
  const dispatch = useDispatch();
  const user = useSelector((store) => store.user);

  const [newAvatar, setNewAvatar] = useState("");
  const [newAvatarObj, setNewAvatarObj] = useState();

  useEffect(() => {
    if (user.changeAvatar.changed && !user.changeAvatar.error) {
      dispatch(fetchUser());
      setNewAvatar("");
      setNewAvatarObj();
    }
  }, [dispatch, user, user.changeAvatar.changed, user.changeAvatar.error]);

  const updateAvatar = useCallback(() => {
    if (!newAvatarObj) return;

    dispatch(changeAvatar(newAvatarObj));
  }, [dispatch, newAvatarObj]);

  const removeAvatar = useCallback(() => {
    dispatch(delAvatar());
  }, [dispatch]);

  const clearNewAvatarUpload = useCallback(() => {
    setNewAvatar("");
  }, []);

  const uploadNewPic = useCallback(() => {
    setNewAvatar("");

    const input = document.createElement("input");

    input.type = "file";
    input.accept = "image/png, image/jpeg";

    input.addEventListener("change", () => {
      if (!input.files[0]) return;

      setNewAvatarObj(input.files[0]);

      setNewAvatar(URL.createObjectURL(input.files[0]));
    });

    input.click();
  }, []);

  return (
    <section>
      <h2>Avatar</h2>
      {newAvatar && <img src={newAvatar} alt="New avatar" />}
      <div className="options">
        {newAvatar && (
          <Button onClick={updateAvatar}>Save as new avatar</Button>
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
        {!newAvatar && user.info.picture && (
          <Button type="secondary" onClick={removeAvatar}>
            Remove current avatar
          </Button>
        )}
      </div>
    </section>
  );
}

export default Avatar;
