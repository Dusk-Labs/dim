import { useCallback, useState } from "react";
import { useSelector } from "react-redux";
import Button from "../../../Components/Misc/Button";

function Avatar() {
  const user = useSelector(store => store.user);

  const [newAvatar, setNewAvatar] = useState("");

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

  return (
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
  );
}

export default Avatar;
